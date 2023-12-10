using Client.Connectivity;
using Unity.VectorGraphics;
using UnityEngine;

namespace UI.Menus.Base
{
    [RequireComponent(typeof(SVGImage))]
    public class ConnectionUI : MonoBehaviour
    {
        [SerializeField] private Connection _connection;
        private Color _connectedColor;
        [SerializeField] private Color _notConnectedColor = Color.gray;

        private SVGImage _svgImage;

        private void Start()
        {
            _svgImage = GetComponent<SVGImage>();
            _connectedColor = _svgImage.color;
        }

        private void Update()
        {
            _svgImage.color = _connection && _connection.IsConnected() ? _connectedColor : _notConnectedColor;
        }
    }
}