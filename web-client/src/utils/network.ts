// TODO: use this to warn before closing window if there are network requests in flight

window.addEventListener('beforeunload', function(e) {
    const confirmationMessage = 'waiting for network';
    // for non-webkit-based browsers:
    (e || window.event).returnValue = confirmationMessage;
    // for webkit-based browsers:
    return confirmationMessage;
});
