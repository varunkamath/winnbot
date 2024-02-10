import sys
import cloudscraper
import json

def get_rank(username, platform, prox_file="./data/prox/proxies_anonymous/http.txt"):
    url = f"https://api.tracker.gg/api/v2/rocket-league/standard/profile/{platform}/{username}"
    # Use proxy: 104.233.55.5:3199
    scraper = cloudscraper.create_scraper()
    # Check if prox_file is a path or a list of proxies by checking if it ends with .txt
    print(f"Prox_file: {prox_file}")
    # Try not using a proxy first
    try:
        print("Trying without proxy...")
        response = scraper.get(url, timeout=1)
        if response.status_code == 200:
            return response.text
        else:
            print(f"Status code: {response.status_code}")
            print(f"Response: {response.text}")
    except Exception as e:
        print(f"Error: {e}")
    if prox_file.endswith(".txt"):
        with open(prox_file, "r") as file:
            proxies = file.readlines()
            for proxy in proxies:
                proxy = proxy.strip()
                print(f"Using proxy: {proxy}")
                try:
                    response = scraper.get(url, proxies={"http": f"http://{proxy}", "https": f"http://{proxy}"}, timeout=1)
                    if response.status_code == 200:
                        break
                    else:
                        continue
                except Exception as e:
                    print(f"Error: {e}")
                    continue
    else:
        print("Proxies are a list")
        for proxy in prox_file:
            print(f"Using proxy: {proxy}")
            try:
                response = scraper.get(url, proxies={"http": f"http://{proxy}", "https": f"http://{proxy}"}, timeout=5)
                if response.status_code == 200:
                    break
                else:
                    continue
            except Exception as e:
                print(f"Error: {e}")
                continue
    # Interate through the list of proxies and use the first one that works. File is named http.txt. If response takes longer than 5 seconds, move to the next proxy.
    print(f"Status code: {response.status_code}")
    print(f"Response: {response.text}")
    if response.status_code == 200:
        return response.text
    else:
        return "Error fetching rank information"
    # response = scraper.get(url, proxies={"http": "http://204.109.59.194:3121", "https": "http://204.109.59.194:3121"}) # "http://165.16.55.19:44444", "https": "http://165.16.55.19:44444"})
    # print("In get_rank function...")
    # print(f"Status code: {response.status_code}")
    # print(f"Response: {response.text}")
    # if response.status_code == 200:
    #     # with open("response.json", "w") as file:
    #     #     file.write(response.text)
    #     # print(response.text)
    #     return response.text
    # else:
    #     return "Error fetching rank information"

def main():
    if len(sys.argv) < 3:
        print("Usage: script.py <username> <platform>")
        return
    
    username = sys.argv[1]
    platform = sys.argv[2]
    prox_file = sys.argv[3] if len(sys.argv) > 3 else "./data/prox/proxies_anonymous/http.txt"
    print(f"Fetching rank for {username} on {platform}...")

    rank = get_rank(username, platform, prox_file)
    print(f"Rank: {rank}")

if __name__ == "__main__":
    main()