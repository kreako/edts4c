import requests


def url(endpoint):
    return f"http://127.0.0.1:8000/api/components/{endpoint}"


def domain_url(endpoint):
    return f"http://127.0.0.1:8000/api/domains/{endpoint}"


def test_components_by_domain_id():
    # Get the first domain
    response = requests.get(domain_url("all"))
    j = response.json()
    domain = j["domains"][0]

    # Get components
    response = requests.get(url(f"by_domain_id/{domain['id']}"))
    assert response.status_code == 200
    j = response.json()

    # Check
    assert j["domain"] == domain
    for component in j["components"]:
        assert component["domain_id"] == domain["id"]
