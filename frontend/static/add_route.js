const elements = {
    form: Page().getElement("#form"),
    alertError: Page().getElement("#alert-error"),
    submitButton: Page().getElement("#submit-button"),
    file: Page().getElement("#file"),
};

Form().onSubmit(elements.form, (formData, formReady) => {

    function prepareData(file) {
        return {
            domain: formData.domain,
            path: formData.path,
            dataUrl: file.dataUrl,
            key: formData.key,
        };
    }

    function addRoute(data) {
        return Api().put("/api/sites", data)
            .then(Api().rejectErrors)
            .then(res => res.json());
    }

    function redirect(data) {
        window.location.href = data.manageUrl;
    }

    function handleError(err) {
        return ErrorMessage().prepare(err).then(msg => {
            Page().showAlert(elements.alertError, msg);
        });
    }

    function beforeSubmit(data) {
        elements.submitButton.disabled = true;

        return data;
    }

    function afterSubmit() {
        formReady();
        elements.submitButton.disabled = false;
    }

    File().onLoad(elements.file)
        .then(beforeSubmit)
        .then(prepareData)
        .then(addRoute)
        .then(redirect)
        .catch(handleError)
        .catch(handleError)
        .finally(afterSubmit);
});
