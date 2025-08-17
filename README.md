>Notes for me:</br>
>IDK what tokio is for</br>



# A Step - by - step instruction on how I set up the Axum Server

1. Create new project
    ```
        cargo new axum_sample
    ```
2. Add axum and tokio in the dependencies in .toml file 
    ```
        tokio = { version = "1", features = ["full"] } 
        axum = "0.7"
    ```
3.In the main.rs, create a route