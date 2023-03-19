(function(METADATA) {
    function err(e) {
        console.error(METADATA["NAME"].toUpperCase() + ".ERROR." + e);
    }
    const ERRORS = {
        API_LOAD_ERROR: "Error loading API",
        API_LINK_ERROR: "Error linking API",
        CLIENT_LOAD_ERROR: "Error loading client code",
        CLIENT_LINK_ERROR: "Error Linking client code",
        CLIENT_RUN_ERROR: "Error running client code",
        LOADER_ERROR: "Internal loader error"
    }
    try {
        const ROOT_URL = "http://localhost:8080/";
        let r = new XMLHttpRequest();
        let res = "";
        r.onreadystatechange = () => {
            if (r.readyState == 4) {
                if (r.status == 200) {
                    res = r.responseText;
                } else {
                    return err(ERRORS.API_LOAD_ERROR);
                }
            }
        }
        r.open("GET", ROOT_URL + "api.js", false);
        r.send();
        try {
            eval(res);
        } catch (e) {
            if(METADATA.DEV_DEBUG)console.warn(e);
            return err(ERRORS.API_LINK_ERROR);
        }
        r = new XMLHttpRequest();
        res = "";
        r.onreadystatechange = () => {
            if (r.readyState == 4) {
                if (r.status == 200) {
                    res = r.responseText;
                } else {
                    return err(ERRORS.CLIENT_LOAD_ERROR);
                }
            }
        }
        r.open("GET", ROOT_URL + "client.js", false);
        r.send();
        try {
            eval(res);
        } catch (e) {
            if(METADATA.DEV_DEBUG)console.warn(e);
            return err(ERRORS.CLIENT_LINK_ERROR);
        }
        try {
            TITANIUM["__run_main__"]();
        } catch (e) {
            if(METADATA.DEV_DEBUG)console.warn(e);
            return err(ERRORS.CLIENT_RUN_ERROR);
        }
    } catch(e) {
        if(METADATA.DEV_DEBUG)console.warn(e);
        return err(ERRORS.LOADER_ERROR);
    }
})({
    NAME: "Titanium",
    VERSION: "1.0.0-alpha.1",
    DEV_DEBUG: true
});