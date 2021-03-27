console.log('[LOADED]')
// document.addEventListener()


function on_submit () {
    console.log('on_submit()')
    inputs = document.querySelectorAll('input')
    console.log(inputs)
    value1_raw = inputs[0].value
    value2_raw = inputs[1].value

    if (value1_raw == '' || value2_raw == '') {
        console.log('empty!')
        return
    }

    value1 = parseInt(value1_raw)
    value2 = parseInt(value2_raw)
    resultp = document.querySelector('p')
    console.log(resultp)
    console.log(resultp)
    resultp.innerHTML = value1 + value2
}