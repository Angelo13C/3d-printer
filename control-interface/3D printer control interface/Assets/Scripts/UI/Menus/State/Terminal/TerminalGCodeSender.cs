using System.Net.Http;
using Client;
using TMPro;
using UnityEngine;

namespace UI.Menus.State.Terminal
{
    public class TerminalGCodeSender : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
        
        [Space]
        [SerializeField] private TMP_InputField _sendCommandField;

        [System.Serializable]
        private struct SendGCodeRequest
        {
            public string gcode;
        }

        public async void SendCommand()
        {
            var stringToSend = _sendCommandField.text;
            _sendCommandField.text = "";

            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Post,
                Content = new StringContent(JsonUtility.ToJson(new SendGCodeRequest
                {
                    gcode = stringToSend
                }))
            };
            await _httpsClient.SendRequestGetRawResponse(request, RequestType.SendGCodeCommands);
        }
    }
}