import requests as req
from time import time
import json as js

def main():
    url = "http://localhost:8000"
    json = {
        "id": "d111",
        "did": "did:iota:test:DLkyWU3jJFgK81KUB3YaDqkwQGMcFNYXTBzj8R4Qhopr",
        "psw": "ciao"
    }
    res = req.post(f"{url}/id-manager/authenticate", json=json)
    cred = res.json()
    print(cred)

    res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    print(res.text)

    json = {
        "cred": js.dumps(cred),
        "channel_psw": "psw"
    }
    res = req.post(f"{url}/channel-manager/daily-channel", headers=json, json={"day_timestamp": int(time()) + 60*60*24})
    print(res.text)

    res = req.get(f"{url}/channel-manager/daily-channel/29-08-2021", headers=json)
    print(res.text)

if __name__ == '__main__':
    main()
