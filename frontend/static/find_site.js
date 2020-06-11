(function() {

    const elements = {
        form: orri.page.getElement("#form"),
        submitButton: orri.page.getElement("#submit-button"),
        alertError: orri.page.getElement("#alert-error"),
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

})();
