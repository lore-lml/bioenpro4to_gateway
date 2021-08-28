import requests as req
from time import time

def main():
    url = "http://localhost:8000"
    headers = {
        "id": "D111",
        "did": "did:iota:test:DLkyWU3jJFgK81KUB3YaDqkwQGMcFNYXTBzj8R4Qhopr",
        "psw": "ciao"
    }
    res = req.get(f"{url}/id-manager/channel-credential", headers=headers)
    cred = res.json()
    print(cred)

    res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    print(res.text)

    # json = {
    #     "cred": cred,
    #     "day_timestamp": int(time()),
    #     "psw": "psw"
    # }
    # res = req.get(f"{url}/channel-manager/daily-channel", json=json)
    # print(res.text)

if __name__ == '__main__':
    main()
