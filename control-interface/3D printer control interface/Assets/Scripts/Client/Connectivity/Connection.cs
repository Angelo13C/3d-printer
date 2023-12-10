using System;
using System.Net.Http;
using System.Threading.Tasks;
using UnityEngine;

namespace Client.Connectivity
{
    public abstract class Connection : MonoBehaviour
    {
        public abstract bool IsConnected();
        public abstract Task<HttpResponseMessage> SendRequest(HttpRequestMessage request, string relativeUri);
    }
}
