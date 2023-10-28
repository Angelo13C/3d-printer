using System.Net.Http;
using Client;
using TMPro;
using UI.Menus.State.Temperature;
using UnityEngine;

namespace UI.Menus.State
{
    public class PrinterState : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
        
        [Header("Configuration")]
        [SerializeField] private float _intervalBetweenPrinterStateUpdates = 2f;
        
        [Header("Temperature")]
        [SerializeField] private TemperaturesGraph _temperaturesGraph;
        [SerializeField] private TMP_InputField _hotendCurrentTemperature;
        [SerializeField] private TMP_InputField _hotendTargetTemperature;
        [SerializeField] private TMP_InputField _bedCurrentTemperature;
        [SerializeField] private TMP_InputField _bedTargetTemperature;

        [System.Serializable]
        public struct HttpResponse
        {
            public int hotendCurrentTemperature;
            public int hotendTargetTemperature;
            public int bedCurrentTemperature;
            public int bedTargetTemperature;
        }

        private void Awake()
        {
            InvokeRepeating(nameof(UpdatePrinterState), 0, _intervalBetweenPrinterStateUpdates);
        }

        private async void UpdatePrinterState()
        {
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Get
            };
            var (hasResponse, response) = await _httpsClient.SendRequest<HttpResponse>(request, RequestType.PrinterState);
            if (!hasResponse)
                return;

            _hotendCurrentTemperature.text = response.hotendCurrentTemperature.ToString();
            _hotendTargetTemperature.text = response.hotendTargetTemperature.ToString();
            _bedCurrentTemperature.text = response.bedCurrentTemperature.ToString();
            _bedTargetTemperature.text = response.bedTargetTemperature.ToString();
            _temperaturesGraph.PlotTemperatures(response.hotendCurrentTemperature, response.bedCurrentTemperature);
        }
    }
}