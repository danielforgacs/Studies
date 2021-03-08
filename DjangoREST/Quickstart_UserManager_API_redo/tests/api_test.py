import requests



def test_GET_root_url():
    response = requests.get('http://localhost:8000')
    assert response.status_code == 200
