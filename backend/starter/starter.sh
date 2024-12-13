init(){
    cargo sqlx database create
    cargo sqlx migrate run
    echo "Running backend service..."
    ./target/release/kleah-backend -r
}

init