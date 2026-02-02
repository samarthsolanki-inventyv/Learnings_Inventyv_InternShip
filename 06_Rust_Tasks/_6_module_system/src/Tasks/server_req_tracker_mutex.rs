use std::sync::Mutex;
// Defining the Request enum
enum Request{
    Get{endpoint : String },
    Post{endpoint: String,payload_size: u32},
    Delete(u32),
}
// Struct to hold request counts
struct RequestCounters{
    get_count:usize,
    post_count:usize,
    delete_count:usize,
}
impl RequestCounters{
    fn new() -> Self {
        RequestCounters {
            get_count: 0,
            post_count: 0,
            delete_count: 0,
        }
    }
    fn total(&self) -> usize {
        self.get_count + self.post_count + self.delete_count
    }
}
 static REQUEST_COUNTERS: Mutex<RequestCounters> = Mutex::new(RequestCounters{
    get_count:0,
    post_count:0,
    delete_count:0
 });
 // Function to handle requests
 fn handle_request(req: Request) -> String {
    match req {
        Request::Get { endpoint } => {
            let mut counters = REQUEST_COUNTERS.lock().unwrap();
            counters.get_count += 1;
            format!("GET request Received on {}", endpoint)
        }
        Request::Post { endpoint, payload_size } => {
            let mut counters = REQUEST_COUNTERS.lock().unwrap();
            counters.post_count += 1;
            format!(
                "POST request to '{}' with payload size {}",
                endpoint, payload_size
            )            
        }
        Request::Delete(id) => {
            let mut counters = REQUEST_COUNTERS.lock().unwrap();
            counters.delete_count += 1;
            format!("DELETE request for resource ID {}", id)
        }
    }
}
fn get_total_requests() -> usize {
    let counters = REQUEST_COUNTERS.lock().unwrap();
    counters.total()
}


pub fn Start_Execute() {
   let get_req = Request::Get{
    endpoint: String::from("/users")
   };
   let post_req = Request::Post{
    endpoint: String::from("/users/add"),
    payload_size: 256,
   };
   let delete_req = Request::Delete(1);
   let get_response = handle_request(get_req);
   let post_response = handle_request(post_req);
   let delete_response = handle_request(delete_req);
   println!("Get Response {}",get_response);
   println!("Post Response {}",post_response);
   println!("Delete Response {}",delete_response);
   println!("-----------------Total request Procesed-----------------");
   println!("{}",get_total_requests());
}
