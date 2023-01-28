use speedire::kubectl_setup::Kubectl;
use speedire::toolfs;
use speedire::kubectl_setup::Tool;

fn main() {
    match toolfs::initialize() {
        Ok(_) => println!("speedire initialized successfully"),
        Err(e) => panic!("error initializing speedire {:?}", e),
    }
    
    let kubectl = Kubectl::default();
    match kubectl.configure() {
        Ok(_) => println!("kubectl configured successfully"),
        Err(e) => panic!("error downloading kubectl {:?}", e),
    }
}
