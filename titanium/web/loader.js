(function(METADATA){
    /**
     * Possible preset errors
     * @property {string} API_LOAD_ERROR Error loading API
     * @property {string} API_LINK_ERROR Error linking API
     * @property {string} CLIENT_LOAD_ERROR Error loading client code
     * @property {string} CLIENT_LINK_ERROR Error Linking client code
     * @property {string} CLIENT_RUN_ERROR Error running client code
     * @property {string} LOADER_ERROR Internal loader error
     * @property {string} UNKNOWN_ERROR Unknown error
    */
    const __TITANIUM_ERRORS__ = {
        API_LOAD_ERROR: "Error loading API",
        API_LINK_ERROR: "Error linking API",
        CLIENT_LOAD_ERROR: "Error loading client code",
        CLIENT_LINK_ERROR: "Error Linking client code",
        CLIENT_RUN_ERROR: "Error running client code",
        LOADER_ERROR: "Internal loader error",
        UNKNOWN_ERROR: "Unknown error"
    };
    /**
     * A simple error handler
     * @param {Error} e the actual error object
     * @param {string} d a basic but descriptive error message
     * @returns {void}
    */
    function err(e, d) {
        if(METADATA.DEV_DEBUG) console.warn(e);
        else console.error(`${METADATA.NAME.toUpperCase()}.ERROR.${d}`);
    }
    /**
     * A simple try-catch wrapper
     * @param {function} callback the function to run
     * @param {string} err the error to throw if there is one
     * @returns {void}
    */
    function run(callback, err) {
        try {
            callback();
        } catch (e) {
            err(e, err);
        }
    }
    /**
     * Creates an XML request and then evals the result
     * @param {string} url the url to request
     * @param {string} err1 the error to throw if the request fails
     * @param {string} err2 the error to throw if the eval fails
     * @returns {void}
    */
    function runXMLRequest(url, err1, err2) {
        run(()=>{
            const xmlRequest = new XMLHttpRequest();
            let xmlResult = "";
            xmlRequest.onreadystatechange = () => {
                if (xmlRequest.readyState == 4) {
                    if (xmlRequest.status == 200) {
                        xmlResult = xmlRequest.responseText;
                    } else {
                        return err(
                            "Server either didn't respond, or responded with a non-200 status code.",
                            err1
                        );
                    }
                }
            }
            xmlRequest.open("GET", url, false);
            xmlRequest.send();
            run(() => eval(xmlResult), err2);
        }, __TITANIUM_ERRORS__.LOADER_ERROR);
    }
    // Run the loader
    run(()=>{
        const maybeConfig = window[
            "__" + METADATA.NAME +
            "_LOADER_CONFIG_DEFAULT_API_CDN__"
        ];// can be undefined

        const/**@type {string}*/ROOT_URL = 
            maybeConfig !== undefined ?
            maybeConfig["ROOT_URL"] :
            METADATA.DEFAULT_API_CDN
        ;
        // First, load the API
        runXMLRequest(
            ROOT_URL + "api.js",
            __TITANIUM_ERRORS__.API_LOAD_ERROR,
            __TITANIUM_ERRORS__.API_LINK_ERROR
        );
        // Then, load the client code
        runXMLRequest(
            ROOT_URL + "client.js",
            __TITANIUM_ERRORS__.CLIENT_LOAD_ERROR,
            __TITANIUM_ERRORS__.CLIENT_LINK_ERROR
        );
        // Finally, run the client code
        run(
            () => TITANIUM["__run_main__"](),
            __TITANIUM_ERRORS__.CLIENT_RUN_ERROR
        );
    }, __TITANIUM_ERRORS__.LOADER_ERROR);
})({
    // Common metadata
    NAME: "Titanium",
    VERSION: "1.0.0-alpha.4",

    // Internal metadata
    DEV_DEBUG: false,
    DEFAULT_API_CDN: "http://localhost:8080/"
});