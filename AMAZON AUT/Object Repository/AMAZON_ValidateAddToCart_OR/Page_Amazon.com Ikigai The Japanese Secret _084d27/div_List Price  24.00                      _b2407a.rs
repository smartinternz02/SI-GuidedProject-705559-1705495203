<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_List Price  24.00                      _b2407a</name>
   <tag></tag>
   <elementGuidId>c9383bdf-92da-42cf-b0b8-94f6e9741e65</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='desktop_qualifiedBuyBox']/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.a-section.a-spacing-none.a-padding-none</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>dec71ada-5946-4eb7-94e0-59f39822a283</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>a-section a-spacing-none a-padding-none</value>
      <webElementGuid>ceaf8b10-1083-4eb3-9278-0b89a5dc846e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>                               
                                                    
                                  
                                                                 
                                                                 
                            
                                                     
                             
                                  
                                                   
                                                       
                                  
                                 List Price:  $24.00                             Details          The List Price is the suggested retail price of a new product as provided by a manufacturer, supplier, or seller. Except for books, Amazon will display a List Price if the product was purchased by customers on Amazon or offered by other retailers at or above the List Price in at least the past 90 days. List prices may not necessarily reflect the product's prevailing market price.Learn more      
    .reinventPrice_legalMessage_icon {
        width: 12px;
        fill: #969696;
        vertical-align: middle;
        padding-bottom: 2px;
    }

    .reinventPrice_legalMessage_icon:hover {
        fill: #555555;
    }



    P.when('A', 'a-popover').execute('a-popover-count', function (A) {
        A.declarative('a-popover', 'mouseenter', function() {
            ue.count(&quot;tooltip.popover.opened&quot;, 1);
        });
    });

         Save:   $10.71 (45%)                                      
                                  
                                                                 
                                  
                                                       
                               
                                  
                                                    
                                  
                                                                               
                                  
                                 
                             
                                  
                                                                                                                               
                                  
                                   
        P.when('A').execute(function(A) {
            if (typeof window.agPopOverCallbackHandle === 'undefined') {
                A.on(&quot;a:popover:show:agShipMsgPopover&quot;, function(data) {
                    A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:activated_popover?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
                });
                window.agPopOverCallbackHandle = true;
            }
        });
    
   
        P.when('A').execute(function(A) {
            recordHelpAndNavigate = function(navigateFn) {
                navigateFn();
                A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:viewed_help?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
            };
        });
    
      $22.27 Shipping &amp; Import Fees Deposit to India       Details      Shipping &amp; Fee Details

                     Price       $13.29           AmazonGlobal Shipping            $11.74          
                                        Estimated Import Fees Deposit
                                       $10.53             Total       $35.56          
                                         
                                  
                                                                 
                                  
                                                   
                                    Delivery Tuesday, February 20. Order within 3 hrs 20 mins                               
                                  
                                    
                                
                                
                                    Deliver to Avinash - Mangalagiri 522502‌
                                
                            
                                                          
                               
                                  
                                                                             
          
                                         
function fastTrackCountDown(secondsLeft, messageSectionId) {
    var sectionId = messageSectionId;
    var FT_showAndInCountdown = false;
    var FT_DayString = &quot;day&quot;;
    var FT_DaysString = &quot;days&quot;;
    var FT_HourString = &quot;hr&quot;;
    var FT_HoursString = &quot;hrs&quot;;
    var FT_MinuteString = &quot;min&quot;;
    var FT_MinutesString = &quot;mins&quot;;
    var FT_AndString = &quot;and&quot;;
    var FT_startedWithHour = new Date().getHours();
    var FT_givenSeconds, FT_actualSeconds;
    var timerId;

    function getElementsByClassNameCustom(className) {
        var selectedElements = [];

        if (document.querySelectorAll) {
            var sectionIdElements = document.querySelectorAll(&quot;#&quot; + sectionId);
            for (index = 0; index &lt; sectionIdElements.length; ++index) {
                var elements = sectionIdElements[index].querySelectorAll(&quot;.&quot; + className);
                for(var i = 0; elements &amp;&amp; i &lt; elements.length; i++) {
                    selectedElements.push(elements[i]);
                }
            }
        }

        return selectedElements;
    }
    
    var FT_CurrentDisplayMin;
    var clientServerTimeDrift;
    var firstTimeUpdate = true;
    
    var countdownElements = getElementsByClassNameCustom(&quot;ftCountdownClass&quot;);
    if (countdownElements.length &lt; 1 &amp;&amp; document.getElementById(sectionId) &amp;&amp; document.getElementById(&quot;ftCountdown&quot;)) {
        countdownElements.push(document.getElementById(&quot;ftCountdown&quot;));
    }
    
    function getTimeRemainingString( days, hours, minutes )
    {

        hours = (days * 24) + hours;
        var hourString   =  ( hours == 1 ? FT_HourString : FT_HoursString );
        var minuteString =  ( minutes  == 1 ? FT_MinuteString : FT_MinutesString );
        if (hours == 0) {
            return minutes + &quot; &quot; + minuteString;
        }
        if (minutes == 0) {
          return hours + &quot; &quot; + hourString;
        }
        if (FT_showAndInCountdown) {
          return hours + &quot; &quot; + hourString + &quot; &quot; + FT_AndString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        } else {
          return hours + &quot; &quot; + hourString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        }

    }        
    
    function hideAllFastTrackComponents() {
        if (document.querySelectorAll) {
            var fastTrackComponents = document.querySelectorAll(&quot;#fast-track&quot;);
            var index;
            var shouldHideSections = false;
            if (fastTrackComponents) {
                for (index = 0; index &lt; fastTrackComponents.length; ++index) {
                    if (fastTrackComponents[index].querySelector(&quot;#&quot; + sectionId)) {
                        fastTrackComponents[index].style.display = &quot;none&quot;;
                    } else {
                        shouldHideSections = true;
                    }
                }
                if (shouldHideSections) {
                    var sectionComponents = document.querySelectorAll(&quot;#&quot; + sectionId);
                    if (sectionComponents) {
                        for (index = 0; index &lt; sectionComponents.length; ++index) {
                            sectionComponents[index].style.display = &quot;none&quot;;
                        }
                    }
                }
            }
        }
    }

    function FT_displayCountdown(forceUpdate)
    {
        var FT_remainSeconds = FT_givenSeconds - FT_actualSeconds;
        //for components having outer div &quot;fast-track&quot; hide that component else hide the message sectionId.
        if (FT_remainSeconds &lt; 1) {
            hideAllFastTrackComponents();
        }

      var FT_secondsPerDay = 24 * 60 * 60;
      var FT_daysLong = FT_remainSeconds / FT_secondsPerDay;
      var FT_days = Math.floor(FT_daysLong);
      var FT_hoursLong = (FT_daysLong - FT_days) * 24;
      var FT_hours = Math.floor(FT_hoursLong);
      var FT_minsLong = (FT_hoursLong - FT_hours) * 60;
      var FT_mins = Math.floor(FT_minsLong);
      var FT_secsLong = (FT_minsLong - FT_mins) * 60;
      var FT_secs = Math.floor(FT_secsLong);
          timerId = setTimeout(FT_getTime, 1000);
      var ftCountdown = getTimeRemainingString( FT_days, FT_hours, FT_mins );
      if (countdownElements.length) {
        if (FT_CurrentDisplayMin != FT_mins || forceUpdate || firstTimeUpdate) {
          var i = 0, countdownElement;
          while (countdownElement = countdownElements[i++]) {
            countdownElement.innerHTML = ftCountdown;
          }
          FT_CurrentDisplayMin = FT_mins;
          firstTimeUpdate = false;
        }
      }
    }
    
    function FT_getCountdown(secondsLeft)
    {
      var FT_currentTime = new Date();
      var FT_currentHours = FT_currentTime.getHours();
      var FT_currentMins = FT_currentTime.getMinutes();
      var FT_currentSecs = FT_currentTime.getSeconds();
      FT_givenSeconds = FT_currentHours * 3600 + FT_currentMins * 60 + FT_currentSecs;
      FT_givenSeconds += secondsLeft;
      FT_getTime();
    }
    function FT_getTime()
    {
      var FT_newCurrentTime = new Date();
      var FT_actualHours = FT_newCurrentTime.getHours();
      if (FT_startedWithHour > FT_actualHours) {
        FT_actualHours += 24;
      }
      var FT_actualMins = FT_newCurrentTime.getMinutes();
      var FT_actualSecs = FT_newCurrentTime.getSeconds();
      FT_actualSeconds = FT_actualHours * 3600 + FT_actualMins * 60 + FT_actualSecs;
      FT_displayCountdown();
    }
        FT_getCountdown(secondsLeft);
        return {
          stopTimer : function() {
            clearTimeout(timerId);
          }
        };
}

 
        P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
            var pageState = A.state('ftPageState');
            if (typeof pageState === 'undefined') {
                pageState = {};
            }

            A.state('ftPageState', pageState);
        });
    
                                                       
                                  
                                                 
                                  
                                                                                       
                                  
                                                                                                                      
                                  
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when('A', 'jQuery').execute(function(A, $) {
        $('#desktop_buybox').find('#promoPriceBlockMessage_feature_div').prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                                  
                                                   
                                  
                                                                                                                  In Stock                                                           
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7        8        9        10        11        12        13        14        15        16        17        18        19        20        21        22        23        24        25        26        27        28        29        30        31        32        33        34        35        36        37        38        39        40        41        42        43        44        45        46        47        48        49        50        51        52        53        54        55        56        57        58        59        60        61        62        63        64        65        66        67        68        69        70        71        72        73        74        75        76        77        78        79        80        81        82        83        84        85        86        87        88        89        90        91        92        93        94        95        96        97        98        99        100             Quantity:1                                                                  
                                  
                                                            
                                  
                                 
    
    

           $$13.2913.29  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($13.29x)          $13.29          Subtotal     $$13.2913.29      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                   
                                  
                                                     
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                           Add to Cart                  
    (function(f) {var _np=(window.P._namespace(&quot;DetailPageBuyBoxTemplate&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.now().execute('dp-mark-atc',function(){
            if (typeof window.markFeatureRender === 'function') {
                window.markFeatureRender('atc',{isInteractive:false});
            }
        });
    }));                                
                                  
                                                                                                     {&quot;turboLoadingText&quot;:&quot;Loading your order summary&quot;,&quot;turboWeblab&quot;:&quot;RCX_CHECKOUT_TURBO_DESKTOP_NONPRIME_87784&quot;,&quot;csrfToken&quot;:&quot;136-0253313-0020608&quot;,&quot;holdbackSecondaryPanelsWeblab&quot;:&quot;&quot;,&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;initiateSelector&quot;:&quot;[id^=buy-now-button]&quot;,&quot;additionalWeblabs&quot;:&quot;{\&quot;RCX_CHECKOUT_DISABLE_TURBO_FOR_NPA_EXPERIMENT_543201\&quot;:\&quot;C\&quot;}&quot;,&quot;turboWeblabTreatment&quot;:&quot;T2&quot;,&quot;version&quot;:&quot;2&quot;,&quot;addressId&quot;:&quot;869105202203&quot;}  {&quot;lineItemInputs&quot;:[{&quot;isTurboEligible&quot;:true,&quot;productTitle&quot;:&quot;Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;asin&quot;:&quot;0143130722&quot;,&quot;offerListingId&quot;:&quot;PY%2F+b++%2FqB8TTCADPLfsi90hPrK28xczKQz%2FKvI8syeciywOZvp6U%2FMNFamJCXPyr0lUzy2%2FAJdWLSCYG9P1pIZXVxGKsEa0YZO4b%2FVe%2FZDZtHKio7Lp01jR77MtdxdzDfAmQWEzESq1eZr56tDVag%3D%3D&quot;}],&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;checkoutClientId&quot;:&quot;retailwebsite&quot;,&quot;turboCheckoutUrl&quot;:&quot;/checkout/turbo-initiate?pipelineType=turbo&quot;,&quot;id&quot;:&quot;buy-now-button&quot;,&quot;version&quot;:&quot;2&quot;}      (function(f) {var _np=(window.P._namespace(&quot;TurboClientDetailPage&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when('cf').execute(function executeTurboAssetsLoadTriggerEvent() {
            P.now('turbo-checkout-assets-load-trigger').execute(function(assetsLoadTrigger) {
                if (assetsLoadTrigger) {
                    logTurboCounter(&quot;AssetTriggerDedupe&quot;);
                    return;
                }

                try {
                    P.declare('turbo-checkout-assets-load-trigger', true);
                    logTurboCounter('AssetTrigger');
                } catch (e) {
                    logTurboCounter('AssetTriggerException');
                }
            });

            function logTurboCounter(name) {
                var counter = 'turboCheckout' + name;
                if (window.ue &amp;&amp; window.ue.count) {
                    window.ue.count(counter, 1);
                }
            }
        });
    }));                Buy Now                                        
                                  
                                                    
                                  
                                                               
                                                         
                                                          
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                        
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                              
                                  
                                                       
                                  
                                     
    
        Customer Service 


           
    Amazon.com         
       Customer Service             Amazon.com                                       
                                  
                                                       
                                  
                                                        
                                  
                                                       
                                  
                                                       
                                  
                                                          
                                  
                              
                                    
                                                      
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                          
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                      
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                    
                                  
                                                                                      
                                  
                               Add a gift receipt for easy returns                                 
                                  
                                          
                                  
                                                 
                                  
                                                      
                                  
                                                     
          </value>
      <webElementGuid>cb3c1c4c-c16b-486e-bc05-0bdcf9a77f1a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;desktop_qualifiedBuyBox&quot;)/div[@class=&quot;a-section a-spacing-none a-padding-none&quot;]</value>
      <webElementGuid>fa135c1b-3f6d-4ce3-8e72-ffc3cbf4987e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='desktop_qualifiedBuyBox']/div</value>
      <webElementGuid>1efb4cf1-b70a-45c6-8d88-4bed6bfe4485</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div[2]/div/form/div/div</value>
      <webElementGuid>147eed98-1744-4080-9593-ce75e521e5fa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;                               
                                                    
                                  
                                                                 
                                                                 
                            
                                                     
                             
                                  
                                                   
                                                       
                                  
                                 List Price:  $24.00                             Details          The List Price is the suggested retail price of a new product as provided by a manufacturer, supplier, or seller. Except for books, Amazon will display a List Price if the product was purchased by customers on Amazon or offered by other retailers at or above the List Price in at least the past 90 days. List prices may not necessarily reflect the product&quot; , &quot;'&quot; , &quot;s prevailing market price.Learn more      
    .reinventPrice_legalMessage_icon {
        width: 12px;
        fill: #969696;
        vertical-align: middle;
        padding-bottom: 2px;
    }

    .reinventPrice_legalMessage_icon:hover {
        fill: #555555;
    }



    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;a-popover&quot; , &quot;'&quot; , &quot;).execute(&quot; , &quot;'&quot; , &quot;a-popover-count&quot; , &quot;'&quot; , &quot;, function (A) {
        A.declarative(&quot; , &quot;'&quot; , &quot;a-popover&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function() {
            ue.count(&quot;tooltip.popover.opened&quot;, 1);
        });
    });

         Save:   $10.71 (45%)                                      
                                  
                                                                 
                                  
                                                       
                               
                                  
                                                    
                                  
                                                                               
                                  
                                 
                             
                                  
                                                                                                                               
                                  
                                   
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;).execute(function(A) {
            if (typeof window.agPopOverCallbackHandle === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
                A.on(&quot;a:popover:show:agShipMsgPopover&quot;, function(data) {
                    A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:activated_popover?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
                });
                window.agPopOverCallbackHandle = true;
            }
        });
    
   
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;).execute(function(A) {
            recordHelpAndNavigate = function(navigateFn) {
                navigateFn();
                A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:viewed_help?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
            };
        });
    
      $22.27 Shipping &amp; Import Fees Deposit to India       Details      Shipping &amp; Fee Details

                     Price       $13.29           AmazonGlobal Shipping            $11.74          
                                        Estimated Import Fees Deposit
                                       $10.53             Total       $35.56          
                                         
                                  
                                                                 
                                  
                                                   
                                    Delivery Tuesday, February 20. Order within 3 hrs 20 mins                               
                                  
                                    
                                
                                
                                    Deliver to Avinash - Mangalagiri 522502‌
                                
                            
                                                          
                               
                                  
                                                                             
          
                                         
function fastTrackCountDown(secondsLeft, messageSectionId) {
    var sectionId = messageSectionId;
    var FT_showAndInCountdown = false;
    var FT_DayString = &quot;day&quot;;
    var FT_DaysString = &quot;days&quot;;
    var FT_HourString = &quot;hr&quot;;
    var FT_HoursString = &quot;hrs&quot;;
    var FT_MinuteString = &quot;min&quot;;
    var FT_MinutesString = &quot;mins&quot;;
    var FT_AndString = &quot;and&quot;;
    var FT_startedWithHour = new Date().getHours();
    var FT_givenSeconds, FT_actualSeconds;
    var timerId;

    function getElementsByClassNameCustom(className) {
        var selectedElements = [];

        if (document.querySelectorAll) {
            var sectionIdElements = document.querySelectorAll(&quot;#&quot; + sectionId);
            for (index = 0; index &lt; sectionIdElements.length; ++index) {
                var elements = sectionIdElements[index].querySelectorAll(&quot;.&quot; + className);
                for(var i = 0; elements &amp;&amp; i &lt; elements.length; i++) {
                    selectedElements.push(elements[i]);
                }
            }
        }

        return selectedElements;
    }
    
    var FT_CurrentDisplayMin;
    var clientServerTimeDrift;
    var firstTimeUpdate = true;
    
    var countdownElements = getElementsByClassNameCustom(&quot;ftCountdownClass&quot;);
    if (countdownElements.length &lt; 1 &amp;&amp; document.getElementById(sectionId) &amp;&amp; document.getElementById(&quot;ftCountdown&quot;)) {
        countdownElements.push(document.getElementById(&quot;ftCountdown&quot;));
    }
    
    function getTimeRemainingString( days, hours, minutes )
    {

        hours = (days * 24) + hours;
        var hourString   =  ( hours == 1 ? FT_HourString : FT_HoursString );
        var minuteString =  ( minutes  == 1 ? FT_MinuteString : FT_MinutesString );
        if (hours == 0) {
            return minutes + &quot; &quot; + minuteString;
        }
        if (minutes == 0) {
          return hours + &quot; &quot; + hourString;
        }
        if (FT_showAndInCountdown) {
          return hours + &quot; &quot; + hourString + &quot; &quot; + FT_AndString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        } else {
          return hours + &quot; &quot; + hourString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        }

    }        
    
    function hideAllFastTrackComponents() {
        if (document.querySelectorAll) {
            var fastTrackComponents = document.querySelectorAll(&quot;#fast-track&quot;);
            var index;
            var shouldHideSections = false;
            if (fastTrackComponents) {
                for (index = 0; index &lt; fastTrackComponents.length; ++index) {
                    if (fastTrackComponents[index].querySelector(&quot;#&quot; + sectionId)) {
                        fastTrackComponents[index].style.display = &quot;none&quot;;
                    } else {
                        shouldHideSections = true;
                    }
                }
                if (shouldHideSections) {
                    var sectionComponents = document.querySelectorAll(&quot;#&quot; + sectionId);
                    if (sectionComponents) {
                        for (index = 0; index &lt; sectionComponents.length; ++index) {
                            sectionComponents[index].style.display = &quot;none&quot;;
                        }
                    }
                }
            }
        }
    }

    function FT_displayCountdown(forceUpdate)
    {
        var FT_remainSeconds = FT_givenSeconds - FT_actualSeconds;
        //for components having outer div &quot;fast-track&quot; hide that component else hide the message sectionId.
        if (FT_remainSeconds &lt; 1) {
            hideAllFastTrackComponents();
        }

      var FT_secondsPerDay = 24 * 60 * 60;
      var FT_daysLong = FT_remainSeconds / FT_secondsPerDay;
      var FT_days = Math.floor(FT_daysLong);
      var FT_hoursLong = (FT_daysLong - FT_days) * 24;
      var FT_hours = Math.floor(FT_hoursLong);
      var FT_minsLong = (FT_hoursLong - FT_hours) * 60;
      var FT_mins = Math.floor(FT_minsLong);
      var FT_secsLong = (FT_minsLong - FT_mins) * 60;
      var FT_secs = Math.floor(FT_secsLong);
          timerId = setTimeout(FT_getTime, 1000);
      var ftCountdown = getTimeRemainingString( FT_days, FT_hours, FT_mins );
      if (countdownElements.length) {
        if (FT_CurrentDisplayMin != FT_mins || forceUpdate || firstTimeUpdate) {
          var i = 0, countdownElement;
          while (countdownElement = countdownElements[i++]) {
            countdownElement.innerHTML = ftCountdown;
          }
          FT_CurrentDisplayMin = FT_mins;
          firstTimeUpdate = false;
        }
      }
    }
    
    function FT_getCountdown(secondsLeft)
    {
      var FT_currentTime = new Date();
      var FT_currentHours = FT_currentTime.getHours();
      var FT_currentMins = FT_currentTime.getMinutes();
      var FT_currentSecs = FT_currentTime.getSeconds();
      FT_givenSeconds = FT_currentHours * 3600 + FT_currentMins * 60 + FT_currentSecs;
      FT_givenSeconds += secondsLeft;
      FT_getTime();
    }
    function FT_getTime()
    {
      var FT_newCurrentTime = new Date();
      var FT_actualHours = FT_newCurrentTime.getHours();
      if (FT_startedWithHour > FT_actualHours) {
        FT_actualHours += 24;
      }
      var FT_actualMins = FT_newCurrentTime.getMinutes();
      var FT_actualSecs = FT_newCurrentTime.getSeconds();
      FT_actualSeconds = FT_actualHours * 3600 + FT_actualMins * 60 + FT_actualSecs;
      FT_displayCountdown();
    }
        FT_getCountdown(secondsLeft);
        return {
          stopTimer : function() {
            clearTimeout(timerId);
          }
        };
}

 
        P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
            var pageState = A.state(&quot; , &quot;'&quot; , &quot;ftPageState&quot; , &quot;'&quot; , &quot;);
            if (typeof pageState === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
                pageState = {};
            }

            A.state(&quot; , &quot;'&quot; , &quot;ftPageState&quot; , &quot;'&quot; , &quot;, pageState);
        });
    
                                                       
                                  
                                                 
                                  
                                                                                       
                                  
                                                                                                                      
                                  
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jQuery&quot; , &quot;'&quot; , &quot;).execute(function(A, $) {
        $(&quot; , &quot;'&quot; , &quot;#desktop_buybox&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#promoPriceBlockMessage_feature_div&quot; , &quot;'&quot; , &quot;).prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                                  
                                                   
                                  
                                                                                                                  In Stock                                                           
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7        8        9        10        11        12        13        14        15        16        17        18        19        20        21        22        23        24        25        26        27        28        29        30        31        32        33        34        35        36        37        38        39        40        41        42        43        44        45        46        47        48        49        50        51        52        53        54        55        56        57        58        59        60        61        62        63        64        65        66        67        68        69        70        71        72        73        74        75        76        77        78        79        80        81        82        83        84        85        86        87        88        89        90        91        92        93        94        95        96        97        98        99        100             Quantity:1                                                                  
                                  
                                                            
                                  
                                 
    
    

           $$13.2913.29  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($13.29x)          $13.29          Subtotal     $$13.2913.29      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                   
                                  
                                                     
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                           Add to Cart                  
    (function(f) {var _np=(window.P._namespace(&quot;DetailPageBuyBoxTemplate&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.now().execute(&quot; , &quot;'&quot; , &quot;dp-mark-atc&quot; , &quot;'&quot; , &quot;,function(){
            if (typeof window.markFeatureRender === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
                window.markFeatureRender(&quot; , &quot;'&quot; , &quot;atc&quot; , &quot;'&quot; , &quot;,{isInteractive:false});
            }
        });
    }));                                
                                  
                                                                                                     {&quot;turboLoadingText&quot;:&quot;Loading your order summary&quot;,&quot;turboWeblab&quot;:&quot;RCX_CHECKOUT_TURBO_DESKTOP_NONPRIME_87784&quot;,&quot;csrfToken&quot;:&quot;136-0253313-0020608&quot;,&quot;holdbackSecondaryPanelsWeblab&quot;:&quot;&quot;,&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;initiateSelector&quot;:&quot;[id^=buy-now-button]&quot;,&quot;additionalWeblabs&quot;:&quot;{\&quot;RCX_CHECKOUT_DISABLE_TURBO_FOR_NPA_EXPERIMENT_543201\&quot;:\&quot;C\&quot;}&quot;,&quot;turboWeblabTreatment&quot;:&quot;T2&quot;,&quot;version&quot;:&quot;2&quot;,&quot;addressId&quot;:&quot;869105202203&quot;}  {&quot;lineItemInputs&quot;:[{&quot;isTurboEligible&quot;:true,&quot;productTitle&quot;:&quot;Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;asin&quot;:&quot;0143130722&quot;,&quot;offerListingId&quot;:&quot;PY%2F+b++%2FqB8TTCADPLfsi90hPrK28xczKQz%2FKvI8syeciywOZvp6U%2FMNFamJCXPyr0lUzy2%2FAJdWLSCYG9P1pIZXVxGKsEa0YZO4b%2FVe%2FZDZtHKio7Lp01jR77MtdxdzDfAmQWEzESq1eZr56tDVag%3D%3D&quot;}],&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;checkoutClientId&quot;:&quot;retailwebsite&quot;,&quot;turboCheckoutUrl&quot;:&quot;/checkout/turbo-initiate?pipelineType=turbo&quot;,&quot;id&quot;:&quot;buy-now-button&quot;,&quot;version&quot;:&quot;2&quot;}      (function(f) {var _np=(window.P._namespace(&quot;TurboClientDetailPage&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when(&quot; , &quot;'&quot; , &quot;cf&quot; , &quot;'&quot; , &quot;).execute(function executeTurboAssetsLoadTriggerEvent() {
            P.now(&quot; , &quot;'&quot; , &quot;turbo-checkout-assets-load-trigger&quot; , &quot;'&quot; , &quot;).execute(function(assetsLoadTrigger) {
                if (assetsLoadTrigger) {
                    logTurboCounter(&quot;AssetTriggerDedupe&quot;);
                    return;
                }

                try {
                    P.declare(&quot; , &quot;'&quot; , &quot;turbo-checkout-assets-load-trigger&quot; , &quot;'&quot; , &quot;, true);
                    logTurboCounter(&quot; , &quot;'&quot; , &quot;AssetTrigger&quot; , &quot;'&quot; , &quot;);
                } catch (e) {
                    logTurboCounter(&quot; , &quot;'&quot; , &quot;AssetTriggerException&quot; , &quot;'&quot; , &quot;);
                }
            });

            function logTurboCounter(name) {
                var counter = &quot; , &quot;'&quot; , &quot;turboCheckout&quot; , &quot;'&quot; , &quot; + name;
                if (window.ue &amp;&amp; window.ue.count) {
                    window.ue.count(counter, 1);
                }
            }
        });
    }));                Buy Now                                        
                                  
                                                    
                                  
                                                               
                                                         
                                                          
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                        
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                              
                                  
                                                       
                                  
                                     
    
        Customer Service 


           
    Amazon.com         
       Customer Service             Amazon.com                                       
                                  
                                                       
                                  
                                                        
                                  
                                                       
                                  
                                                       
                                  
                                                          
                                  
                              
                                    
                                                      
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                          
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                      
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                    
                                  
                                                                                      
                                  
                               Add a gift receipt for easy returns                                 
                                  
                                          
                                  
                                                 
                                  
                                                      
                                  
                                                     
          &quot;) or . = concat(&quot;                               
                                                    
                                  
                                                                 
                                                                 
                            
                                                     
                             
                                  
                                                   
                                                       
                                  
                                 List Price:  $24.00                             Details          The List Price is the suggested retail price of a new product as provided by a manufacturer, supplier, or seller. Except for books, Amazon will display a List Price if the product was purchased by customers on Amazon or offered by other retailers at or above the List Price in at least the past 90 days. List prices may not necessarily reflect the product&quot; , &quot;'&quot; , &quot;s prevailing market price.Learn more      
    .reinventPrice_legalMessage_icon {
        width: 12px;
        fill: #969696;
        vertical-align: middle;
        padding-bottom: 2px;
    }

    .reinventPrice_legalMessage_icon:hover {
        fill: #555555;
    }



    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;a-popover&quot; , &quot;'&quot; , &quot;).execute(&quot; , &quot;'&quot; , &quot;a-popover-count&quot; , &quot;'&quot; , &quot;, function (A) {
        A.declarative(&quot; , &quot;'&quot; , &quot;a-popover&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function() {
            ue.count(&quot;tooltip.popover.opened&quot;, 1);
        });
    });

         Save:   $10.71 (45%)                                      
                                  
                                                                 
                                  
                                                       
                               
                                  
                                                    
                                  
                                                                               
                                  
                                 
                             
                                  
                                                                                                                               
                                  
                                   
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;).execute(function(A) {
            if (typeof window.agPopOverCallbackHandle === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
                A.on(&quot;a:popover:show:agShipMsgPopover&quot;, function(data) {
                    A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:activated_popover?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
                });
                window.agPopOverCallbackHandle = true;
            }
        });
    
   
        P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;).execute(function(A) {
            recordHelpAndNavigate = function(navigateFn) {
                navigateFn();
                A.ajax(&quot;https://fls-na.amazon.com/1/action-impressions/1/OE/amazon-global/action/amazon_global_shipmsg_:viewed_help?marketplaceId=ATVPDKIKX0DER&amp;requestId=FT1TV27VG3CHC12MZSTZ&amp;session=136-0253313-0020608&quot;, { method: &quot;get&quot; });
            };
        });
    
      $22.27 Shipping &amp; Import Fees Deposit to India       Details      Shipping &amp; Fee Details

                     Price       $13.29           AmazonGlobal Shipping            $11.74          
                                        Estimated Import Fees Deposit
                                       $10.53             Total       $35.56          
                                         
                                  
                                                                 
                                  
                                                   
                                    Delivery Tuesday, February 20. Order within 3 hrs 20 mins                               
                                  
                                    
                                
                                
                                    Deliver to Avinash - Mangalagiri 522502‌
                                
                            
                                                          
                               
                                  
                                                                             
          
                                         
function fastTrackCountDown(secondsLeft, messageSectionId) {
    var sectionId = messageSectionId;
    var FT_showAndInCountdown = false;
    var FT_DayString = &quot;day&quot;;
    var FT_DaysString = &quot;days&quot;;
    var FT_HourString = &quot;hr&quot;;
    var FT_HoursString = &quot;hrs&quot;;
    var FT_MinuteString = &quot;min&quot;;
    var FT_MinutesString = &quot;mins&quot;;
    var FT_AndString = &quot;and&quot;;
    var FT_startedWithHour = new Date().getHours();
    var FT_givenSeconds, FT_actualSeconds;
    var timerId;

    function getElementsByClassNameCustom(className) {
        var selectedElements = [];

        if (document.querySelectorAll) {
            var sectionIdElements = document.querySelectorAll(&quot;#&quot; + sectionId);
            for (index = 0; index &lt; sectionIdElements.length; ++index) {
                var elements = sectionIdElements[index].querySelectorAll(&quot;.&quot; + className);
                for(var i = 0; elements &amp;&amp; i &lt; elements.length; i++) {
                    selectedElements.push(elements[i]);
                }
            }
        }

        return selectedElements;
    }
    
    var FT_CurrentDisplayMin;
    var clientServerTimeDrift;
    var firstTimeUpdate = true;
    
    var countdownElements = getElementsByClassNameCustom(&quot;ftCountdownClass&quot;);
    if (countdownElements.length &lt; 1 &amp;&amp; document.getElementById(sectionId) &amp;&amp; document.getElementById(&quot;ftCountdown&quot;)) {
        countdownElements.push(document.getElementById(&quot;ftCountdown&quot;));
    }
    
    function getTimeRemainingString( days, hours, minutes )
    {

        hours = (days * 24) + hours;
        var hourString   =  ( hours == 1 ? FT_HourString : FT_HoursString );
        var minuteString =  ( minutes  == 1 ? FT_MinuteString : FT_MinutesString );
        if (hours == 0) {
            return minutes + &quot; &quot; + minuteString;
        }
        if (minutes == 0) {
          return hours + &quot; &quot; + hourString;
        }
        if (FT_showAndInCountdown) {
          return hours + &quot; &quot; + hourString + &quot; &quot; + FT_AndString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        } else {
          return hours + &quot; &quot; + hourString + &quot; &quot; + minutes + &quot; &quot; + minuteString;
        }

    }        
    
    function hideAllFastTrackComponents() {
        if (document.querySelectorAll) {
            var fastTrackComponents = document.querySelectorAll(&quot;#fast-track&quot;);
            var index;
            var shouldHideSections = false;
            if (fastTrackComponents) {
                for (index = 0; index &lt; fastTrackComponents.length; ++index) {
                    if (fastTrackComponents[index].querySelector(&quot;#&quot; + sectionId)) {
                        fastTrackComponents[index].style.display = &quot;none&quot;;
                    } else {
                        shouldHideSections = true;
                    }
                }
                if (shouldHideSections) {
                    var sectionComponents = document.querySelectorAll(&quot;#&quot; + sectionId);
                    if (sectionComponents) {
                        for (index = 0; index &lt; sectionComponents.length; ++index) {
                            sectionComponents[index].style.display = &quot;none&quot;;
                        }
                    }
                }
            }
        }
    }

    function FT_displayCountdown(forceUpdate)
    {
        var FT_remainSeconds = FT_givenSeconds - FT_actualSeconds;
        //for components having outer div &quot;fast-track&quot; hide that component else hide the message sectionId.
        if (FT_remainSeconds &lt; 1) {
            hideAllFastTrackComponents();
        }

      var FT_secondsPerDay = 24 * 60 * 60;
      var FT_daysLong = FT_remainSeconds / FT_secondsPerDay;
      var FT_days = Math.floor(FT_daysLong);
      var FT_hoursLong = (FT_daysLong - FT_days) * 24;
      var FT_hours = Math.floor(FT_hoursLong);
      var FT_minsLong = (FT_hoursLong - FT_hours) * 60;
      var FT_mins = Math.floor(FT_minsLong);
      var FT_secsLong = (FT_minsLong - FT_mins) * 60;
      var FT_secs = Math.floor(FT_secsLong);
          timerId = setTimeout(FT_getTime, 1000);
      var ftCountdown = getTimeRemainingString( FT_days, FT_hours, FT_mins );
      if (countdownElements.length) {
        if (FT_CurrentDisplayMin != FT_mins || forceUpdate || firstTimeUpdate) {
          var i = 0, countdownElement;
          while (countdownElement = countdownElements[i++]) {
            countdownElement.innerHTML = ftCountdown;
          }
          FT_CurrentDisplayMin = FT_mins;
          firstTimeUpdate = false;
        }
      }
    }
    
    function FT_getCountdown(secondsLeft)
    {
      var FT_currentTime = new Date();
      var FT_currentHours = FT_currentTime.getHours();
      var FT_currentMins = FT_currentTime.getMinutes();
      var FT_currentSecs = FT_currentTime.getSeconds();
      FT_givenSeconds = FT_currentHours * 3600 + FT_currentMins * 60 + FT_currentSecs;
      FT_givenSeconds += secondsLeft;
      FT_getTime();
    }
    function FT_getTime()
    {
      var FT_newCurrentTime = new Date();
      var FT_actualHours = FT_newCurrentTime.getHours();
      if (FT_startedWithHour > FT_actualHours) {
        FT_actualHours += 24;
      }
      var FT_actualMins = FT_newCurrentTime.getMinutes();
      var FT_actualSecs = FT_newCurrentTime.getSeconds();
      FT_actualSeconds = FT_actualHours * 3600 + FT_actualMins * 60 + FT_actualSecs;
      FT_displayCountdown();
    }
        FT_getCountdown(secondsLeft);
        return {
          stopTimer : function() {
            clearTimeout(timerId);
          }
        };
}

 
        P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
            var pageState = A.state(&quot; , &quot;'&quot; , &quot;ftPageState&quot; , &quot;'&quot; , &quot;);
            if (typeof pageState === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
                pageState = {};
            }

            A.state(&quot; , &quot;'&quot; , &quot;ftPageState&quot; , &quot;'&quot; , &quot;, pageState);
        });
    
                                                       
                                  
                                                 
                                  
                                                                                       
                                  
                                                                                                                      
                                  
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jQuery&quot; , &quot;'&quot; , &quot;).execute(function(A, $) {
        $(&quot; , &quot;'&quot; , &quot;#desktop_buybox&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#promoPriceBlockMessage_feature_div&quot; , &quot;'&quot; , &quot;).prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                                  
                                                   
                                  
                                                                                                                  In Stock                                                           
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7        8        9        10        11        12        13        14        15        16        17        18        19        20        21        22        23        24        25        26        27        28        29        30        31        32        33        34        35        36        37        38        39        40        41        42        43        44        45        46        47        48        49        50        51        52        53        54        55        56        57        58        59        60        61        62        63        64        65        66        67        68        69        70        71        72        73        74        75        76        77        78        79        80        81        82        83        84        85        86        87        88        89        90        91        92        93        94        95        96        97        98        99        100             Quantity:1                                                                  
                                  
                                                            
                                  
                                 
    
    

           $$13.2913.29  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($13.29x)          $13.29          Subtotal     $$13.2913.29      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                   
                                  
                                                     
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                           Add to Cart                  
    (function(f) {var _np=(window.P._namespace(&quot;DetailPageBuyBoxTemplate&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.now().execute(&quot; , &quot;'&quot; , &quot;dp-mark-atc&quot; , &quot;'&quot; , &quot;,function(){
            if (typeof window.markFeatureRender === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
                window.markFeatureRender(&quot; , &quot;'&quot; , &quot;atc&quot; , &quot;'&quot; , &quot;,{isInteractive:false});
            }
        });
    }));                                
                                  
                                                                                                     {&quot;turboLoadingText&quot;:&quot;Loading your order summary&quot;,&quot;turboWeblab&quot;:&quot;RCX_CHECKOUT_TURBO_DESKTOP_NONPRIME_87784&quot;,&quot;csrfToken&quot;:&quot;136-0253313-0020608&quot;,&quot;holdbackSecondaryPanelsWeblab&quot;:&quot;&quot;,&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;initiateSelector&quot;:&quot;[id^=buy-now-button]&quot;,&quot;additionalWeblabs&quot;:&quot;{\&quot;RCX_CHECKOUT_DISABLE_TURBO_FOR_NPA_EXPERIMENT_543201\&quot;:\&quot;C\&quot;}&quot;,&quot;turboWeblabTreatment&quot;:&quot;T2&quot;,&quot;version&quot;:&quot;2&quot;,&quot;addressId&quot;:&quot;869105202203&quot;}  {&quot;lineItemInputs&quot;:[{&quot;isTurboEligible&quot;:true,&quot;productTitle&quot;:&quot;Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;asin&quot;:&quot;0143130722&quot;,&quot;offerListingId&quot;:&quot;PY%2F+b++%2FqB8TTCADPLfsi90hPrK28xczKQz%2FKvI8syeciywOZvp6U%2FMNFamJCXPyr0lUzy2%2FAJdWLSCYG9P1pIZXVxGKsEa0YZO4b%2FVe%2FZDZtHKio7Lp01jR77MtdxdzDfAmQWEzESq1eZr56tDVag%3D%3D&quot;}],&quot;turboHeaderText&quot;:&quot;Buy now: Ikigai: The Japanese Secret to a Long and Happy Life&quot;,&quot;checkoutClientId&quot;:&quot;retailwebsite&quot;,&quot;turboCheckoutUrl&quot;:&quot;/checkout/turbo-initiate?pipelineType=turbo&quot;,&quot;id&quot;:&quot;buy-now-button&quot;,&quot;version&quot;:&quot;2&quot;}      (function(f) {var _np=(window.P._namespace(&quot;TurboClientDetailPage&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when(&quot; , &quot;'&quot; , &quot;cf&quot; , &quot;'&quot; , &quot;).execute(function executeTurboAssetsLoadTriggerEvent() {
            P.now(&quot; , &quot;'&quot; , &quot;turbo-checkout-assets-load-trigger&quot; , &quot;'&quot; , &quot;).execute(function(assetsLoadTrigger) {
                if (assetsLoadTrigger) {
                    logTurboCounter(&quot;AssetTriggerDedupe&quot;);
                    return;
                }

                try {
                    P.declare(&quot; , &quot;'&quot; , &quot;turbo-checkout-assets-load-trigger&quot; , &quot;'&quot; , &quot;, true);
                    logTurboCounter(&quot; , &quot;'&quot; , &quot;AssetTrigger&quot; , &quot;'&quot; , &quot;);
                } catch (e) {
                    logTurboCounter(&quot; , &quot;'&quot; , &quot;AssetTriggerException&quot; , &quot;'&quot; , &quot;);
                }
            });

            function logTurboCounter(name) {
                var counter = &quot; , &quot;'&quot; , &quot;turboCheckout&quot; , &quot;'&quot; , &quot; + name;
                if (window.ue &amp;&amp; window.ue.count) {
                    window.ue.count(counter, 1);
                }
            }
        });
    }));                Buy Now                                        
                                  
                                                    
                                  
                                                               
                                                         
                                                          
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                        
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                              
                                  
                                                       
                                  
                                     
    
        Customer Service 


           
    Amazon.com         
       Customer Service             Amazon.com                                       
                                  
                                                       
                                  
                                                        
                                  
                                                       
                                  
                                                       
                                  
                                                          
                                  
                              
                                    
                                                      
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
                                     
                                                     
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                          
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                      
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                    
                                  
                                                                                      
                                  
                               Add a gift receipt for easy returns                                 
                                  
                                          
                                  
                                                 
                                  
                                                      
                                  
                                                     
          &quot;))]</value>
      <webElementGuid>c3004a78-489c-46de-a15d-adde1fbc5616</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
