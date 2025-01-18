async function crawl(website_id) {
  fetch("/crawl",
    {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "POST",
      body: JSON.stringify({"id":website_id})
    });
}

async function update(website_id) {
  let website_data = fetch("/website_data?id="+website_id,
    {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "GET"
    }).then(response => {
      return response.json()
    });
  console.log(website_data);
}
