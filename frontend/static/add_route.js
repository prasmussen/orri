Form().onSubmit(document.getElementById("site"), data => {
    File().onLoad(document.getElementById("file"), file => {
        if (!file) {
            console.log("Empty file");
            return;
        }

        data.dataUrl = file.dataUrl;

        Api().put("/api/sites", data).then(res => {
            console.log(res);
        });
    });
});
