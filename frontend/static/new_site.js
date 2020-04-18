Form().onSubmit(document.getElementById("site"), formData => {

    function prepareData(file) {
        const domain = [formData.subdomain, formData.mainDomain].join(".");
        const key = Crypto().randomString(32);

        return {
            domain: domain,
            key: key,
            dataUrl: file.dataUrl
        };
    }

    function createSite(data) {
        return Api().post("/api/sites", data)
            .then(Api().rejectErrors)
            .then(res => res.json())
            .then(json => Object.assign(data, json));
    }

    function showSuccessPage(data) {
        // Populate placeholder values
        document.getElementById("key-placeholder").innerText = data.key;
        document.getElementById("domain-placeholder").innerText = data.domain;
        document.getElementById("site-url-placeholder").href = data.siteUrl;
        document.getElementById("manage-url-placeholder").href = data.manageUrl;

        // Switch to success view
        document.getElementById("main-content").classList.add("display-none");
        document.getElementById("success-content").classList.remove("display-none");

        return null;
    }

    function handleError(err) {
        return ErrorMessage().prepare(err).then(msg => {
            Page().showAlert(document.getElementById("alert-error"), msg);
        });
    }

    File().onLoad(document.getElementById("file"))
        .then(prepareData)
        .then(createSite)
        .then(showSuccessPage)
        .catch(handleError)
        .catch(handleError);
});
