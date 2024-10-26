/*
    http://soundfile.sapp.org/doc/WaveFormat/
    http://www.topherlee.com/software/pcm-tut-wavformat.html
    https://docs.fileformat.com/audio/wav/

    Example sine wave:
    https://www3.nd.edu/~dthain/courses/cse20211/fall2013/wavfile/
*/

type FourBytes = [u8; 4];

use std::fs::{File, write};
use std::io::{BufWriter, Write};

const RIFF_ASCII: FourBytes = [0x52, 0x49, 0x46, 0x46];
const WAVE_ASCII: FourBytes = [0x57, 0x41, 0x56, 0x45];
const FMT_ASCII: FourBytes = [0x66, 0x6D, 0x74, 0x20];
const DATA_ASCII: FourBytes = [0x64, 0x61, 0x74, 0x61];
const BITS_PER_SAMPLE: u32 = 16;

enum AudioFormat {
    PCM,
}

enum FormatDataLength {
    U16,
}

enum ChannelNum {
    Mono,
    Stereo,
}

enum SampleRate {
    Half,
    Cd,
}

struct Wav {
    riff_descriptor: FourBytes,
    file_type_header: FourBytes,
    format_chunk_marker: FourBytes,
    length_of_format_data: FormatDataLength,
    audio_format: AudioFormat,
    num_channels: ChannelNum,
    sample_rate: SampleRate,
    samples: Vec<u16>,
}

impl AudioFormat {
    fn to_bytes(&self) -> [u8; 2] {
        match self {
            &Self::PCM => [1_u8, 0_u8],
        }
    }
}

impl FormatDataLength {
    fn to_bytes(&self) -> FourBytes {
        match self {
            &Self::U16 => bytes_from_u32_LE(&16_u32)
        }
    }
}

impl ChannelNum {
    fn to_bytes(&self) -> [u8; 2] {
        match self {
            &Self::Mono => [1_u8, 0_u8],
            &Self::Stereo => [2_u8, 0_u8],
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            &Self::Mono => 1,
            &Self::Stereo => 2,
        }
    }
}

impl SampleRate {
    fn to_bytes(&self) -> FourBytes {
        match self {
            Self::Half => bytes_from_u32_LE(&22050),
            Self::Cd => bytes_from_u32_LE(&44100),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::Half => 22050,
            Self::Cd => 44100,
        }
    }
}

impl Wav {
    fn new(samples: Vec<u16>) -> Self {
        Self {
            riff_descriptor: RIFF_ASCII,
            file_type_header: WAVE_ASCII,
            format_chunk_marker: FMT_ASCII,
            length_of_format_data: FormatDataLength::U16,
            audio_format: AudioFormat::PCM,
            num_channels: ChannelNum::Stereo,
            sample_rate: SampleRate::Cd,
            samples,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.riff_descriptor);
        data.extend([0u8; 4]);
        data.extend(self.file_type_header);
        data.extend(self.format_chunk_marker);
        data.extend(self.length_of_format_data.to_bytes());
        data.extend(self.audio_format.to_bytes());
        data.extend(self.num_channels.to_bytes());
        data.extend(self.sample_rate.to_bytes());
        data.extend(bytes_from_u32_LE(&self.get_byte_rate()));
        assert_eq!(data.len(), 32);
        data.extend(self.get_block_align());
        data.extend(self.get_bits_per_sample());
        assert_eq!(data.len(), 36);
        data.extend(DATA_ASCII);
        // data.extend([0u8; 4]);
        data.extend(bytes_from_u32_BE(&(self.samples.len() as u32)));
        assert_eq!(data.len(), 44);
        for sample in &self.samples {
            data.extend([
                *sample as u8,
                (*sample >> 8) as u8,
            ])
        }
        data
    }

    fn get_byte_rate(&self) -> u32 {
        (self.sample_rate.to_u32() * BITS_PER_SAMPLE * self.num_channels.to_u32()) / 8
    }

    fn get_block_align(&self) -> [u8; 2] {
        let block_align = self.num_channels.to_u32() * BITS_PER_SAMPLE / 8;
        [
            (block_align) as u8,
            (block_align >> 8) as u8,
        ]
    }

    fn get_bits_per_sample(&self) -> [u8; 2] {
        [
            (BITS_PER_SAMPLE) as u8,
            (BITS_PER_SAMPLE >> 8) as u8,
        ]
    }
}

fn bytes_from_u32_LE(data: &u32) -> FourBytes {
    [
        (data >> 0) as u8,
        (data >> 8) as u8,
        (data >> 16) as u8,
        (*data as u64 >> 32) as u8,
    ]
}

fn bytes_from_u32_BE(data: &u32) -> FourBytes {
    [
        (*data as u64 >> 32) as u8,
        (data >> 16) as u8,
        (data >> 8) as u8,
        (data >> 0) as u8,
    ]
}

fn main() {
    let samples = gen_samples();
    let wav = Wav::new(samples);
    let mut file = std::fs::File::create("generated.wav").unwrap();
    file.write(&wav.as_bytes()).unwrap();
}

fn gen_samples() -> Vec<u16> {
    let num_samples = 22050 * 5;
    let mut samples: Vec<u16> = Vec::with_capacity(num_samples);
    for index in 0_usize..num_samples {
        let sample_value = ((index as f64).sin() + 1.0) / 2.0;
        let sample_value = (sample_value * std::u16::MAX as f64) as u16;
        println!("sample: {}", sample_value);
        samples.push(sample_value);
        samples.push(sample_value);
    }
    samples
}
