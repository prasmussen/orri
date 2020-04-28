(function() {

    const elements = {
        removeRouteButton: orri.page.getElement("#remove-route"),
        alertError: orri.page.getElement("#alert-error"),
    };

    orri.button.onClick(elements.removeRouteButton, (buttonBodyData, buttonReady) => {

        function redirect(data) {
            window.location.href = data.manageUrl;
        }

        function handleError(err) {
            orri.page.showError(elements.alertError, err);
        }

        if (!window.confirm("Do you really want to delete this route?")) {
            buttonReady();
            return;
        }

        return orri.api.request(elements.removeRouteButton.dataset.apiMethod, elements.removeRouteButton.dataset.apiUrl, buttonBodyData)
            .then(orri.api.rejectErrors)
            .then(res => res.json())
            .then(redirect)
            .catch(handleError)
            .catch(handleError)
            .finally(buttonReady);
    });

})()
