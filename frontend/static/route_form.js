(function() {

    const elements = {
        form: orri.page.getElement("#form"),
        alertError: orri.page.getElement("#alert-error"),
        submitButton: orri.page.getElement("#submit-button"),
        file: orri.page.getElement("#file"),
    };

    orri.form.onSubmit(elements.form, elements.submitButton, (formData, formReady) => {

        function prepareData(file) {
            return {
                domain: formData.domain,
                path: formData.path,
                dataUrl: file.dataUrl,
                key: formData.key,
            };
        }

        function updateRoute(data) {
            return orri.api.request(elements.form.dataset.apiMethod, elements.form.dataset.apiUrl, data)
                .then(orri.api.rejectErrors)
                .then(res => res.json());
        }

        function redirect(data) {
            window.location.href = data.manageUrl;
        }

        function handleError(err) {
            orri.page.showError(elements.alertError, err);
        }

        orri.file.onLoad(elements.file)
            .then(prepareData)
            .then(updateRoute)
            .then(redirect)
            .catch(handleError)
            .catch(handleError)
            .finally(formReady);
    });

})();
