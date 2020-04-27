const elements = {
    removeSiteButton: orri.page.getElement("#remove-site"),
    alertError: orri.page.getElement("#alert-error"),
};

orri.button.onClick(elements.removeSiteButton, (buttonBodyData, buttonReady) => {

    function redirect() {
        // TODO: redirect to /sites
        window.location.href = "/";
    }

    function handleError(err) {
        orri.page.showError(elements.alertError, err);
    }

    if (!window.confirm("Do you really want to delete this site?")) {
        buttonReady();
        return;
    }

    return orri.api.request(elements.removeSiteButton.dataset.apiMethod, elements.removeSiteButton.dataset.apiUrl, buttonBodyData)
        .then(orri.api.rejectErrors)
        .then(redirect)
        .catch(handleError)
        .catch(handleError)
        .finally(buttonReady);
});
