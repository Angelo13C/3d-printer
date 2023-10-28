using UnityEngine;
using UnityEngine.UI;

namespace UI.Menus.State.Terminal
{
    [RequireComponent(typeof(ScrollRect))]
    public class Autoscroll : MonoBehaviour
    {
        [SerializeField] private float _lerpSpeed = 0.2f;
        
        private ScrollRect _scrollRect;
        
        private bool _enabled = true;

        public void SetEnabled(bool enabled) => _enabled = enabled;

        private void Awake()
        {
            _scrollRect = GetComponent<ScrollRect>();
        }

        private void Update()
        {
            if(!_enabled)
                return;

            _scrollRect.verticalNormalizedPosition = Mathf.Lerp(_scrollRect.verticalNormalizedPosition, 0, _lerpSpeed);
        }
    }
}