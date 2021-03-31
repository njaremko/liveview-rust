const path = require('path');

module.exports = {
    mode: 'production',
    entry: './static/js/liveview-dev.js',
    output: {
        filename: 'liveview.js',
        path: path.resolve(__dirname, 'static/js')
    }
};