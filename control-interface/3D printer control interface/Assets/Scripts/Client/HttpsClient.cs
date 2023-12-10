using System.Collections.Generic;
using System.Net.Http;
using System.Security.Authentication;
using System.Threading.Tasks;
using Client.Connectivity;
using JetBrains.Annotations;
using UnityEngine;

namespace Client
{
    public class HttpsClient : MonoBehaviour
    {
        public HttpClient InnerClient { get; private set; }

        private List<Connection> _connections = new List<Connection>(2);

        public void AddConnection(Connection connection) => _connections.Add(connection);
    
        private void Awake()
        {
            var handler = new HttpClientHandler()
            {
                SslProtocols = SslProtocols.Tls12 | SslProtocols.Tls11 | SslProtocols.Tls
            };
            InnerClient = new HttpClient(handler);
        }

        [CanBeNull]
        public async Task<(bool, T)> SendRequest<T>(HttpRequestMessage request, RequestType requestType)
        {
            var response = await SendRequestGetRawResponse(request, requestType);
            if (response == null || response.Content == null)
                return (false, default);
            
            var responseBodyString = await response.Content.ReadAsStringAsync();
            return (true, JsonUtility.FromJson<T>(responseBodyString));
        }

        [CanBeNull]
        public async Task<HttpResponseMessage> SendRequestGetRawResponse(HttpRequestMessage request, RequestType requestType)
        {
            foreach (var connection in _connections)
            {
                if (connection.IsConnected())
                {
                    return await connection.SendRequest(request, requestType.ToUri());
                }
            }

            return null;
        }
    }   
}
