<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getoranization</name>
   <tag></tag>
   <elementGuidId>8d9b53fd-bba4-4ed4-8f83-92167df1dfcf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: []
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>f4fa07df-8588-435e-a1f0-8303c9c61dbf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.token}</value>
      <webElementGuid>432a63eb-1a0f-40ec-a018-00351b3ba67c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseurl}/api/v1/organization</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'data.id', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data.name', &quot;Sekolah Digital Cilsy&quot;)
WS.verifyElementPropertyValue(response, 'data.taxId', null)
WS.verifyElementPropertyValue(response, 'data.registraionNumber', null)
WS.verifyElementPropertyValue(response, 'data.phone', null)
WS.verifyElementPropertyValue(response, 'data.fax', null)
WS.verifyElementPropertyValue(response, 'data.email', null)
WS.verifyElementPropertyValue(response, 'data.country', &quot;ID&quot;)
WS.verifyElementPropertyValue(response, 'data.province', null)
WS.verifyElementPropertyValue(response, 'data.city', null)
WS.verifyElementPropertyValue(response, 'data.zipCode', null)
WS.verifyElementPropertyValue(response, 'data.street1', null)
WS.verifyElementPropertyValue(response, 'data.street2', null)
WS.verifyElementPropertyValue(response, 'data.note', null)
WS.verifyElementPropertyValue(response, 'data.numberOfEmployees', 20)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
