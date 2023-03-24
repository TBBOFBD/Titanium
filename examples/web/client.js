const sleep = ms => new Promise(r => setTimeout(r, ms));

TITANIUM.main(async ()=>{
    const server = TITANIUM.connect();

    let _connected = false;

    server.on("open",()=>{
        _connected = true;
        console.log("Connected to server!");
    });

    while(true){
        if(_connected)break;
        await sleep(10);
    }

    server.send("Hello World!")

});
