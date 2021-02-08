import requests


def url(endpoint):
    return f"http://127.0.0.1:8000/api/domains/{endpoint}"


def test_domains_all():
    response = requests.get(url("all"))
    assert response.status_code == 200
    j = response.json()
    assert "domains" in j


def test_domains_set_title():
    # initial
    response = requests.get(url("all"))
    j = response.json()
    original = j["domains"][0]

    # patch
    response = requests.put(url(f"set_title/{original['id']}"), json={"title": "meuh"})
    assert response.status_code == 200

    # check
    response = requests.get(url("all"))
    j = response.json()
    altered = j["domains"][0]
    assert original["id"] == altered["id"]
    assert altered["title"] == "meuh"

    # go back
    response = requests.put(url(f"set_title/{original['id']}"), json={"title": original["title"]})
    assert response.status_code == 200

    # check
    response = requests.get(url("all"))
    j = response.json()
    altered = j["domains"][0]
    assert original["id"] == altered["id"]
    assert altered["title"] == original["title"]


def test_domains_set_domains_rank():
    # initial
    response = requests.get(url("all"))
    j = response.json()
    domains = j["domains"]

    # patch by reordering first and last
    tmp = domains[0]
    domains[0] = domains[-1]
    domains[-1] = tmp
    response = requests.put(url("set_domains_rank"), json={"domains": domains})
    assert response.status_code == 200

    # check
    response = requests.get(url("all"))
    j = response.json()
    check = j["domains"]
    assert len(domains) == len(check)
    for i, domain in enumerate(check):
        assert domain["id"] == domains[i]["id"]
        assert domain["rank"] == i + 1
        assert domain["title"] == domains[i]["title"]

    # patch back
    tmp = domains[0]
    domains[0] = domains[-1]
    domains[-1] = tmp
    response = requests.put(url("set_domains_rank"), json={"domains": domains})
    assert response.status_code == 200

    # check
    response = requests.get(url("all"))
    j = response.json()
    check = j["domains"]
    assert len(domains) == len(check)
    for i, domain in enumerate(check):
        assert domain == domains[i]


def test_domains_append_delete():
    # original
    response = requests.get(url("all"))
    j = response.json()
    original = j["domains"]

    # append
    response = requests.post(url("append"), json={"title": "meuh"})
    assert response.status_code == 200

    # Check
    response = requests.get(url("all"))
    j = response.json()
    domains = j["domains"]
    assert len(original) == len(domains) - 1
    assert domains[-1]["title"] == "meuh"
    assert domains[-1]["rank"] == len(domains)

    # Back with delete
    response = requests.delete(url(f"delete/{domains[-1]['id']}"))
    assert response.status_code == 200

    # Check
    response = requests.get(url("all"))
    j = response.json()
    domains = j["domains"]
    assert len(domains) == len(original)
    for i, domain in enumerate(domains):
        assert domain == original[i]
