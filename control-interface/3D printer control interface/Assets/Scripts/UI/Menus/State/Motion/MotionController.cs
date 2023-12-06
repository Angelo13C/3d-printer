using System.Net.Http;
using Client;
using UnityEngine;

namespace UI.Menus.State.Motion
{
    public class MotionController : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
        
        public void MoveX(float direction) => MoveOnAxis(Axis.X, direction);
        public void MoveY(float direction) => MoveOnAxis(Axis.Y, direction);
        public void MoveZ(float direction) => MoveOnAxis(Axis.Z, direction);

        private enum Axis
        {
            X,
            Y,
            Z
        }

        [System.Serializable]
        private struct MoveRequest
        {
            public string axis;
            public float direction;
        }

        private async void MoveOnAxis(Axis axis, float direction)
        {
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Post,
                Content = new StringContent(JsonUtility.ToJson(new MoveRequest
                {
                    direction = direction * Time.deltaTime,
                    axis = axis.ToString()
                }))
            };
            await _httpsClient.SendRequestGetRawResponse(request, RequestType.Move);
        }
    }
}