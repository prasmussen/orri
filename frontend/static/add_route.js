Form().onSubmit(document.getElementById("site"), (formData, formReady) => {

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
            Page().showAlert(document.getElementById("alert-error"), msg);
        });
    }

    function setButtonDisabled(isDisabled) {
        document.getElementById("submit-button").disabled = isDisabled;
    }

    function beforeSubmit(data) {
        setButtonDisabled(true);

        return data;
    }

    function afterSubmit() {
        formReady();
        setButtonDisabled(false);
    }

    File().onLoad(document.getElementById("file"))
        .then(beforeSubmit)
        .then(prepareData)
        .then(addRoute)
        .then(redirect)
        .catch(handleError)
        .catch(handleError)
        .finally(afterSubmit);
});
