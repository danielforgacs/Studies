import pytest
import re

PATTERN = r'(a+)(_(a+))*'

def name_check(input):
    return bool(re.fullmatch(PATTERN, input))


@pytest.mark.parametrize('expected, input', (
    (True, 'a'),
    (True, 'aaa'),
    (True, 'aaa_a'),

    (True, 'aa_aa_aa_aa'),

    (False, 'aa____aa__aa_aa'),

    (False, '_'),
    (False, '_a'),
    (False, '_a_'),
    (False, 'a_'),
))
def test_name_regex(expected, input):
    assert name_check(input) is expected
