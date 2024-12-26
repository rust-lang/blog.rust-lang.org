"use strict";

// The regular theme-switch.js script runs in the header and blocks the initial
// page render to prevent flickering. The following code cannot run as part of
// that, because the page must have been rendered first.

// close the theme dropdown if clicking somewhere else
document.querySelector('.theme-icon').onblur = handleBlur;

// show the theme selector only if JavaScript is enabled/available
document.querySelector('.theme-icon').style.display = 'block';
