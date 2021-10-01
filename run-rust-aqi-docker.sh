# Build and tag container first
# Configure .env file with your API Key and Zipcode
docker run \
    --env-file .env \
    --network="host" \
    --rm \
    rust-aqi-query:v1 

