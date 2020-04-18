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
            .then(res => data);
    }

    function getSuccessHtml(data) {
        return Api().post("/api/sites/site-created", {
            domain: data.domain,
            key: data.key
        })
        .then(Api().rejectErrors)
        .then(res => res.json())
        .then(json => json.html);
    }

    function renderSucessHtml(html) {
        const content = document.getElementById("content");
        content.innerHTML = html;

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
        .then(getSuccessHtml)
        .then(renderSucessHtml)
        .catch(handleError)
        .catch(handleError);
});
