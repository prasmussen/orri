
function Form() {

    function getData(elem) {
        const data = new FormData(elem);
        const result = {};

        data.forEach((value, key) => {
            result[key] = value;
        });

        return result;
    }

    function onSubmit(form, callback) {
        form.addEventListener('submit', event => {
            event.preventDefault();

            var data = getData(form);
            callback(data);
        });

    }

    return {
        onSubmit: onSubmit,
    };
}

function Api() {
    function post(url, data) {
        return fetch(url, {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
    }

    return {
        post: post,
    };
}

(function() {

    Form().onSubmit(document.getElementById('document'), data => {
        Api().post("/api/documents", data).then(res => {
            console.log(res);
        });
    });

})();
