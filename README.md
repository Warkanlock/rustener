# Rustener
## A fast url shortener made in Rust in less than 100 lines.

A lot of this code is just to play with Rust as a tool and see what it is capable of. I did this to understand how things like ``Mutex<>`` and asynchronous references worked. I also tried to do this to put Rocket into practice as my main web framework for web development in Rust.

## How to use it

### **Statistics** (count of urls and version) 

```curl
curl --request GET --url http://127.0.0.1:8000/
```

### **Create** a short url

```curl
curl --request GET \
  --url http://127.0.0.1:8000/new \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "www.yourwebpage.com",
	"creator": "warkanlock"
}'
```

### **Visualize** data from an identifier

```curl
curl --request GET --url http://127.0.0.1:8000/{identifier}
```