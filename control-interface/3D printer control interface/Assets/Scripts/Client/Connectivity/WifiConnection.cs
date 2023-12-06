using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;
using UnityEngine;

namespace Client.Connectivity
{
    [RequireComponent(typeof(HttpsClient))]
    public class WifiConnection : Connection
    {
        [SerializeField] private float _secondsBetweenFindPrinterIpAddressRequests = 1;
        
        const string UriPrefix = "https://";
        const string IpAddressPrefix = "192.168.1.";
        const byte FirstAddress = 1;
        const byte LastAddress = 254;
        
        private List<Task<HttpResponseMessage>> _findPrinterIpAddressRequests = new(LastAddress - FirstAddress + 1);
        private string _printerIPAddress = "";
        
        private HttpsClient _httpsClient;

        private void Start()
        {
            _httpsClient = GetComponent<HttpsClient>();
            _httpsClient.AddConnection(this);

            _httpsClient.InnerClient.Timeout = TimeSpan.FromSeconds(_secondsBetweenFindPrinterIpAddressRequests);
            
            InvokeRepeating(nameof(FindPrinterIpAddressIfNecessary), 0, _secondsBetweenFindPrinterIpAddressRequests);
        }

        public override bool IsConnected()
        {
            return _printerIPAddress != "";
        }

        public override Task<HttpResponseMessage> SendRequest(HttpRequestMessage request, string relativeUri)
        {
            request.RequestUri = new Uri(UriPrefix + _printerIPAddress + relativeUri);
            return _httpsClient.InnerClient.SendAsync(request);
        }

        private void Update()
        {
            for (var i = _findPrinterIpAddressRequests.Count - 1; i >= 0; i--)
            {
                var response = _findPrinterIpAddressRequests[i];
                if (response.IsCompleted)
                {
                    if (response.IsCompletedSuccessfully)
                    {
                        _printerIPAddress = IpAddressPrefix + (i + FirstAddress).ToString();
                        _findPrinterIpAddressRequests.Clear();
                        break;
                    }
                    
                    _findPrinterIpAddressRequests.RemoveAt(i);
                }
            }
        }

        private void FindPrinterIpAddressIfNecessary()
        {
            if (_printerIPAddress != "" || _findPrinterIpAddressRequests.Count > 0)
                return;

            const string UriSuffix = "/find_printer";

            for (var i = FirstAddress; i <= LastAddress; i++)
            {
                var uri = UriPrefix + IpAddressPrefix + i + UriSuffix;
                var response = _httpsClient.InnerClient.SendAsync(new HttpRequestMessage(HttpMethod.Head, uri));
                _findPrinterIpAddressRequests.Add(response);
            }
        }
    }
}
