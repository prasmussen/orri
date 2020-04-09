Form().onSubmit(document.getElementById("document"), data => {
    File().onLoad(document.getElementById("file"), file => {
        if (!file) {
            console.log("Empty file");
            return;
        }

        data.dataUrl = file.dataUrl;

        Api().post("/api/sites", data).then(res => {
            console.log(res);
        });
    });
});
