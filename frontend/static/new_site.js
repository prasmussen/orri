const elements = {
    form: orri.page.getElement("#form"),
    alertError: orri.page.getElement("#alert-error"),
    submitButton: orri.page.getElement("#submit-button"),
    file: orri.page.getElement("#file"),
    keyPlaceholder: orri.page.getElement("#key-placeholder"),
    domainPlaceholder: orri.page.getElement("#domain-placeholder"),
    siteUrlPlaceholder: orri.page.getElement("#site-url-placeholder"),
    manageUrlPlaceholder: orri.page.getElement("#manage-url-placeholder"),
    mainContent: orri.page.getElement("#main-content"),
    successContent: orri.page.getElement("#success-content"),
};

orri.form.onSubmit(elements.form, elements.submitButton, (formData, formReady) => {

    function prepareData(file) {
        const domain = [formData.subdomain, formData.sitesDomain].join(".");
        const key = orri.crypto.randomString(20);

        return {
            domain: domain,
            key: key,
            dataUrl: file.dataUrl
        };
    }

    function createSite(data) {
        return orri.api.post(elements.form.dataset.apiUrl, data)
            .then(orri.api.rejectErrors)
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
        orri.page.hideElement(elements.mainContent);
        orri.page.unhideElement(elements.successContent);

        return null;
    }

    function handleError(err) {
        orri.page.showError(elements.alertError, err);
    }

    orri.file.onLoad(elements.file)
        .then(prepareData)
        .then(createSite)
        .then(showSuccessPage)
        .catch(handleError)
        .catch(handleError)
        .finally(formReady);
});
