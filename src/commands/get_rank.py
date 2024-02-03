import sys
import cloudscraper
import json

def get_rank(username, platform):
    url = f"https://api.tracker.gg/api/v2/rocket-league/standard/profile/{platform}/{username}"
    scraper = cloudscraper.create_scraper()
    response = scraper.get(url)
    print(f"Status code: {response.status_code}")
    if response.status_code == 200:
        # with open("response.json", "w") as file:
        #     file.write(response.text)
        # print(response.text)
        return response.text
    else:
        return "Error fetching rank information"

def main():
    if len(sys.argv) < 3:
        print("Usage: script.py <username> <platform>")
        return
    
    username = sys.argv[1]
    platform = sys.argv[2]
    print(f"Fetching rank for {username} on {platform}...")

    rank = get_rank(username, platform)
    print(f"Rank: {rank}")

if __name__ == "__main__":
    main()