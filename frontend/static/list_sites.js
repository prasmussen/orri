(function() {

    const elements = {
        form: orri.page.getElement("#form"),
        submitButton: orri.page.getElement("#submit-button"),
        alertError: orri.page.getElement("#alert-error"),
        manageOtherButton: orri.page.getElement("#manage-other"),
        manageOtherForm: orri.page.getElement("#manage-other-form"),
        sitesTable: orri.page.getElement("#sites-table"),
        mySitesButton: orri.page.getElement("#show-my-sites"),
    };

    function ensureSiteExists(url) {
        return orri.api.request("HEAD", url)
            .then(orri.api.rejectErrors)
            .then(res => url);
    }

    function redirect(url) {
        window.location.href = url;
    }

    function handleError(err) {
        orri.page.showError(elements.alertError, err);
    }

    orri.form.onSubmit(elements.form, elements.submitButton, (formData, formReady) => {

        const manageUrl = [
            elements.form.dataset.apiBaseUrl,
            "/",
            formData.subdomain,
            ".",
            formData.sitesDomain,
        ].join("");

        ensureSiteExists(manageUrl)
            .then(redirect)
            .catch(handleError)
            .catch(handleError)
            .finally(formReady);
    });

    orri.button.onClick(elements.manageOtherButton, (buttonBodyData, buttonReady) => {
        // Switch to success view
        orri.page.hideElement(elements.alertError);
        orri.page.hideElement(elements.sitesTable);
        orri.page.unhideElement(elements.manageOtherForm);

        buttonReady();
    });

    orri.button.onClick(elements.mySitesButton, (buttonBodyData, buttonReady) => {
        // Switch to success view
        orri.page.hideElement(elements.alertError);
        orri.page.hideElement(elements.manageOtherForm);
        orri.page.unhideElement(elements.sitesTable);

        buttonReady();
    });

})();
