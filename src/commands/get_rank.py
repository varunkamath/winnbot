import sys
import cloudscraper
import json

# Function to get the user's rank
def get_rank(username, platform):
    url = f"https://api.tracker.gg/api/v2/rocket-league/standard/profile/{platform}/{username}"
    scraper = cloudscraper.create_scraper()
    response = scraper.get(url)
    # Print the URL and status code
    print(f"URL: {url}")
    print(f"Status code: {response.status_code}")
    # Check if the response was successful
    if response.status_code == 200:
        # data = response.json()  # Parse the JSON response
        # # Extract rank information (you'll need to adjust the path according to the API's response structure)
        # rank_info = data.get('data', {}).get('segments', [])[0].get('stats', {}).get('tier', {}).get('metadata', {}).get('name', 'Rank not found')
        # return rank_info
        # Save the response to a file named response.json
        with open("response.json", "w") as file:
            file.write(response.text)
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