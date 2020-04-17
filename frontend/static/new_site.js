Form().onSubmit(document.getElementById("site"), data => {
    File().onLoad(document.getElementById("file"), file => {
        if (!file) {
            console.log("Empty file");
            return;
        }

        var key = Crypto().randomString(32);
        var domain = [data.subdomain, data.mainDomain].join(".");
        delete data.subdomain;
        delete data.mainDomain;

        Object.assign(data, {
            key: key,
            domain: domain,
            dataUrl: file.dataUrl,
        });

        Api().post("/api/sites", data).then(res => {
            console.log(res);
        });
    });
});
