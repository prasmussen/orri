const elements = {
    form: Page().getElement("#form"),
    alertError: Page().getElement("#alert-error"),
    submitButton: Page().getElement("#submit-button"),
    file: Page().getElement("#file"),
    keyPlaceholder: Page().getElement("#key-placeholder"),
    domainPlaceholder: Page().getElement("#domain-placeholder"),
    siteUrlPlaceholder: Page().getElement("#site-url-placeholder"),
    manageUrlPlaceholder: Page().getElement("#manage-url-placeholder"),
    mainContent: Page().getElement("#main-content"),
    successContent: Page().getElement("#success-content"),
};

Form().onSubmit(elements.form, elements.submitButton, (formData, formReady) => {

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
        elements.keyPlaceholder.innerText = data.key;
        elements.domainPlaceholder.innerText = data.domain;
        elements.siteUrlPlaceholder.href = data.siteUrl;
        elements.manageUrlPlaceholder.href = data.manageUrl;

        // Switch to success view
        Page().hideElement(elements.mainContent);
        Page().unhideElement(elements.successContent);

        return null;
    }

    function handleError(err) {
        return ErrorMessage().prepare(err).then(msg => {
            Page().showAlert(elements.alertError, msg);
        });
    }

    File().onLoad(elements.file)
        .then(prepareData)
        .then(createSite)
        .then(showSuccessPage)
        .catch(handleError)
        .catch(handleError)
        .finally(formReady);
});
