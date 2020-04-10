Form().onSubmit(document.getElementById("site"), data => {
    File().onLoad(document.getElementById("file"), file => {
        if (!file) {
            console.log("Empty file");
            return;
        }

        var key = Crypto().randomString(32);

        Object.assign(data, {
            key: key,
            dataUrl: file.dataUrl,
        });

        Api().post("/api/sites", data).then(res => {
            console.log(res);
        });
    });
});
