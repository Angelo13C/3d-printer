using System.Net.Http;
using UnityEngine;

namespace Client.Connectivity
{
    public abstract class Connection : MonoBehaviour
    {
        public abstract bool IsConnected();
        protected abstract bool SendIfPossible(HttpRequestMessage request);
    }
}
