async function crawl(website_id) {
    fetch("/crawl",
        {
            headers: {
              'Accept': 'application/json',
              'Content-Type': 'application/json'
            },
            method: "POST",
            body: JSON.stringify({"id":website_id})
        })
}
