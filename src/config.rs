pub struct Config{
    port: i8, // user-defined port <- used for localhost address (127.0.0.1)
    url: String // path for the file that is located within the users system
}

pub fn create_config(){
    // we take the user inputted values and store it into a struct
    // we store the struct data into a text file
    // we break down the struct data into a readable format like json or something
    // or we can make our own format like the below
    /*
        User Input: Port:3434 URL: C:/Desktop/Test/test.html 
        Struct Data: let config1 = Config{3434,"C:/Desktop/Test/test.html"}; <- according to the rust compiler
            
        How we store it in our own format
        Config1(
            Port: 3434, 
            URL: "C:/Desktop/Test/test.html"
        );
    */
}
    
pub fn read_config(){
    // we read our text file containing our configs
    // then we read the data in the file accordingly
    // we take the important information in the text file via parsing 
}
    
pub fn save_config(){
    // we call this function when user wants to save 
    // will be stored into a text file
}
    
pub fn load_config(){
    // we load all the configs saved in the text file
    // let user select one of the configs
    // then when user opens the page at the specified port
}    
    
pub fn default_config(){
    // load the default configuration
    // set by the program
    // port 3000
    // serves the hello.html page and 404.html page for errorinous condition
}