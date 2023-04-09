use titanium::desktop::memory::*;

const OFFSET: usize = 140725876420396;
const APPNAME: &str = "dummy";

const SET_TO: u32 = 6;

fn main() {
    // Injecting ourselves into the process handle (this process, you can also inject into other processes)
    let handle = get_handle(APPNAME).expect("Failed to get process handle");
    // let handle = (std::process::id() as Pid)
    //     .try_into_process_handle()
    //     .expect("Failed to get process handle");

    // We make a `DataMember` that has an offset referring to its location in memory
    let member = DataMember::new_offset(handle, vec![OFFSET]);
    // The memory refered to is now the same
    println!(
        "Memory location: &x: {}, member: {}",
        OFFSET,
        member.get_offset().expect("Failed to get member's offset")
    );
    // The value of the member is the same as the variable
    println!(
        "Member value: {}",
        unsafe {
            member.read().expect("Failed to read member's value")
        }
    );

    // We can write to and modify the value of the variable using the member
    member.write(&SET_TO).unwrap();
}