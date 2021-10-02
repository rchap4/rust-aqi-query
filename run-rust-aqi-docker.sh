# Build and tag container first
# Configure .env file with your API Key and Zipcode
docker run \
    --env-file .env \
    -p 3030:3030 \
    --rm \
    rust-aqi-query:v1
