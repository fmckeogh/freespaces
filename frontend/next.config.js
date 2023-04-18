/** @type {import('next').NextConfig} */

module.exports = {
  async rewrites() {
    return [
      {
        source: '/locations',
        destination: 'http://127.0.0.1:8080/locations'
      }
    ]
  }
}