using System;
using System.Net.Http;
using Client;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace UI.Menus.Base
{
    public class PrintStatus : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
        
        [Space]
        [SerializeField] private GameObject _machineIsPrinting;
        [SerializeField] private GameObject _machineIsNotPrinting;
        
        [Space]
        [SerializeField] private TextMeshProUGUI _textInfo;
        [SerializeField] private TextMeshProUGUI _percentageText;

        [Space]
        [SerializeField] private GameObject _pausedButtonIcon;

        [Header("Percentage bar")]
        [SerializeField] private Image _percentageBarFill;
        [SerializeField] private Color _fillColorWhenPaused = Color.yellow;
        private Color _fillColorWhenNotPaused;

        [Header("Configuration")]
        [SerializeField] private float _intervalBetweenPrintStatusUpdates = 2f;
        
        private void Start()
        {
            _fillColorWhenNotPaused = _percentageBarFill.color;
            
            InvokeRepeating(nameof(UpdatePrintStatus), 0, _intervalBetweenPrintStatusUpdates);
        }

        public async void PauseResumeButtonPressed()
        {
            SetPaused(!_pausedButtonIcon.activeSelf);
            
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Post
            };
            await _httpsClient.SendRequestGetRawResponse(request, RequestType.PauseOrResume);
        }

        private void SetPaused(bool paused)
        {
            _pausedButtonIcon.SetActive(paused);
            var resumeButtonIconIndex = _pausedButtonIcon.transform.GetSiblingIndex() == 0 ? 1 : 0;
            _pausedButtonIcon.transform.parent.GetChild(resumeButtonIconIndex).gameObject.SetActive(!paused);

            _percentageBarFill.color = paused ? _fillColorWhenPaused : _fillColorWhenNotPaused;
        }

        [System.Serializable]
        public struct HttpResponse
        {
            public bool isPrinting;
            public string fileNameBeingPrinted;
            public int printDurationInSeconds;
            public int timePrintedInSeconds;
            public bool isPaused;
        }

        private async void UpdatePrintStatus()
        {
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Get
            };
            var (hasRespone, response) = await _httpsClient.SendRequest<HttpResponse>(request, RequestType.GetPrintStatus);
            if (!hasRespone)
            {
                _machineIsPrinting.SetActive(false);
                _machineIsNotPrinting.SetActive(false);
                return;
            }
            
            _machineIsPrinting.SetActive(response.isPrinting);
            _machineIsNotPrinting.SetActive(!response.isPrinting);
            SetPaused(response.isPaused);
            if (response.isPrinting)
            {
                var remainingTimeInSeconds = response.printDurationInSeconds - response.timePrintedInSeconds;
                var remainingTimeInMinutes = Mathf.CeilToInt(remainingTimeInSeconds / 60f);
                var remainingTimeInHours = remainingTimeInMinutes / 60;
                remainingTimeInMinutes -= remainingTimeInHours * 60;
                var remainingTimeFormatted = remainingTimeInHours > 0
                    ? (remainingTimeInHours == 1 ? "1 hour" : remainingTimeInHours + " hours")
                    : "";
                if (remainingTimeInMinutes > 0)
                {
                    if (remainingTimeInHours > 0)
                        remainingTimeFormatted += " and ";
                    
                    if(remainingTimeInMinutes == 1)
                        remainingTimeFormatted += "1 minute";
                    else
                        remainingTimeFormatted += remainingTimeInMinutes + " minutes";
                }
                
                var printDurationFormatted = TimeSpan.FromSeconds(response.printDurationInSeconds).ToString(@"hh\:mm\:ss");
                var timePrintedFormatted = TimeSpan.FromSeconds(response.timePrintedInSeconds).ToString(@"hh\:mm\:ss");
                _textInfo.text = "Printing file: " + response.fileNameBeingPrinted + "\n"
                    + "Time printed/Total print time: " + timePrintedFormatted + "/" + printDurationFormatted + "\n"
                    + "Remaining time: " + remainingTimeFormatted;

                var percentage = Mathf.FloorToInt(100f * response.timePrintedInSeconds / response.printDurationInSeconds);
                _percentageText.text = percentage + "%";
            }
        }
    }
}