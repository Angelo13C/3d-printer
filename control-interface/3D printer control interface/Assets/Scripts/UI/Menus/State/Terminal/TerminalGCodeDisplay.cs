using System;
using System.Net.Http;
using System.Text;
using Client;
using TMPro;
using UnityEngine;

namespace UI.Menus.State.Terminal
{
    public class TerminalGCodeDisplay : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;

        [Space]
        [SerializeField] private TextMeshProUGUI _display;
        
        [Header("Configuration")]
        [SerializeField] private float _intervalBetweenPrinterStateUpdates = 5f;

        private int _currentLine;

        [System.Serializable]
        private struct HttpRequestData
        {
            public int requestedLine;
        }
        
        [System.Serializable]
        public struct HttpResponse
        {
            public int lineOfFirstCommand;
            public string commands;
        }

        private void Awake()
        {
            InvokeRepeating(nameof(UpdateDisplay), 0f, _intervalBetweenPrinterStateUpdates);
        }

        private async void UpdateDisplay()
        {
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Get,
                Content = new StringContent(JsonUtility.ToJson(new HttpRequestData
                {
                    requestedLine = _currentLine
                }))
            };
            var (hasResponse, response) = await _httpsClient.SendRequest<HttpResponse>(request, RequestType.ListGCodeCommandsInMemory);
            if (!hasResponse)
                return;
            
            var commandLines = response.commands.Split(
                new[] { "\r\n", "\r", "\n" },
                StringSplitOptions.None
            );
            _currentLine = response.lineOfFirstCommand + commandLines.Length;
            var commandLinesInSingleString = new StringBuilder(response.commands.Length + "• \n".Length * commandLines.Length);
            foreach (var commandLine in commandLines)
                commandLinesInSingleString.AppendLine("• " + commandLine);
            _display.text += commandLinesInSingleString;
        }
    }
}